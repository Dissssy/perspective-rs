use futures::StreamExt;

use crate::ClientConfig;

/// A perspective API client, which automatically handles rate limiting and requests.
pub struct Client {
    thread: tokio::task::JoinHandle<()>,
    sender: tokio::sync::mpsc::Sender<crate::types::RequestWithPriority>,
    receiver: Option<tokio::sync::mpsc::Receiver<crate::types::Response>>,
    killer: Option<tokio::sync::oneshot::Sender<()>>,
}

impl Client {
    pub async fn new(config: ClientConfig) -> Self {
        let (req_sender, req_receiver) = tokio::sync::mpsc::channel::<crate::types::RequestWithPriority>(config.request_buffer_size);
        let (res_sender, res_receiver) = tokio::sync::mpsc::channel::<crate::types::Response>(config.response_buffer_size);
        let (killer_sender, killer_receiver) = tokio::sync::oneshot::channel::<()>();

        let thread = tokio::spawn(thread(config, req_receiver, res_sender, killer_receiver));

        Self {
            thread,
            sender: req_sender,
            receiver: Some(res_receiver),
            killer: Some(killer_sender),
        }
    }
    pub async fn send_high(&self, req: crate::types::Request) -> Result<(), tokio::sync::mpsc::error::SendError<crate::types::RequestWithPriority>> {
        self.sender.send(crate::types::RequestWithPriority::High(req)).await
    }
    pub async fn send_normal(&self, req: crate::types::Request) -> Result<(), tokio::sync::mpsc::error::SendError<crate::types::RequestWithPriority>> {
        self.sender.send(crate::types::RequestWithPriority::Normal(req)).await
    }
    pub async fn send_low(&self, req: crate::types::Request) -> Result<(), tokio::sync::mpsc::error::SendError<crate::types::RequestWithPriority>> {
        self.sender.send(crate::types::RequestWithPriority::Low(req)).await
    }
    pub async fn recv(&mut self) -> Option<crate::types::Response> {
        match self.receiver.as_mut() {
            Some(r) => r.recv().await,
            None => Some(Err(crate::types::ApiError::ReceiverTaken)),
        }
    }
    /// if you want to use the receiver in a stream, you can take it
    pub fn take_receiver(&mut self) -> Option<tokio::sync::mpsc::Receiver<crate::types::Response>> {
        self.receiver.take()
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if let Some(killer) = self.killer.take() {
            if let Err(()) = killer.send(()) {
                log::error!("failed to send kill signal");
            }
        }
        self.thread.abort();
    }
}

async fn thread(
    config: ClientConfig,
    mut req_receiver: tokio::sync::mpsc::Receiver<crate::types::RequestWithPriority>,
    res_sender: tokio::sync::mpsc::Sender<crate::types::Response>,
    mut killer_receiver: tokio::sync::oneshot::Receiver<()>,
) {
    let reqwest_client = reqwest::Client::new();
    let mut tick = tokio::time::interval(std::time::Duration::from_millis(config.tick_rate));
    // let mut queue: Vec<crate::types::Request> = Vec::with_capacity(config.maximum_queue_size);
    let mut low_priority_queue = futures::stream::FuturesOrdered::new();
    let mut normal_priority_queue = futures::stream::FuturesOrdered::new();
    let mut high_priority_queue = futures::stream::FuturesOrdered::new();

    loop {
        tokio::select! {
            _ = tick.tick() => {
                // we only want to run a single request every tick
                // if let Some(req) = queue.next().await {
                //     log::info!("sending request");
                //     if let Err(e) = res_sender.send(req).await {
                //         log::error!("failed to send response: {}", e);
                //     }
                // }

                if let Some(req) = high_priority_queue.next().await {
                    log::info!("sending high priority request");
                    if let Err(e) = res_sender.send(req).await {
                        log::error!("failed to send response: {}", e);
                    }
                } else if let Some(req) = normal_priority_queue.next().await {
                    log::info!("sending normal priority request");
                    if let Err(e) = res_sender.send(req).await {
                        log::error!("failed to send response: {}", e);
                    }
                } else if let Some(req) = low_priority_queue.next().await {
                    log::info!("sending low priority request");
                    if let Err(e) = res_sender.send(req).await {
                        log::error!("failed to send response: {}", e);
                    }
                }
            }
            Some(req) = req_receiver.recv() => {
                log::info!("received request");

                match req {
                    crate::types::RequestWithPriority::Low(req) => {
                        if low_priority_queue.len() < config.maximum_queue_size {
                            low_priority_queue.push_back(get_response(
                                req,
                                &reqwest_client,
                                &config.api_key,
                            ));
                        } else {
                            log::info!("low priority queue is full");
                            if let Err(e) = res_sender.send(Err(crate::types::ApiError::QueueFull)).await {
                                log::error!("failed to send queue full error: {}", e);
                            }
                        }
                    }
                    crate::types::RequestWithPriority::Normal(req) => {
                        if normal_priority_queue.len() < config.maximum_queue_size {
                            normal_priority_queue.push_back(get_response(
                                req,
                                &reqwest_client,
                                &config.api_key,
                            ));
                        } else {
                            log::info!("normal priority queue is full");
                            if let Err(e) = res_sender.send(Err(crate::types::ApiError::QueueFull)).await {
                                log::error!("failed to send queue full error: {}", e);
                            }
                        }
                    }
                    crate::types::RequestWithPriority::High(req) => {
                        if high_priority_queue.len() < config.maximum_queue_size {
                            high_priority_queue.push_back(get_response(
                                req,
                                &reqwest_client,
                                &config.api_key,
                            ));
                        } else {
                            log::info!("high priority queue is full");
                            if let Err(e) = res_sender.send(Err(crate::types::ApiError::QueueFull)).await {
                                log::error!("failed to send queue full error: {}", e);
                            }
                        }
                    }
                }

                // if queue.len() < config.maximum_queue_size {
                //     queue.push_back(get_response(
                //         req,
                //         &reqwest_client,
                //         &config.api_key,
                //     ));
                // } else {
                //     log::info!("queue is full");
                //     if let Err(e) = res_sender.send(Err(crate::types::ApiError::QueueFull)).await {
                //         log::error!("failed to send queue full error: {}", e);
                //     }
                // }
            }
            _ = &mut killer_receiver => {
                log::info!("killing thread");
                break;
            }
        }
    }
}

async fn get_response(req: crate::types::Request, client: &reqwest::Client, api_key: &str) -> crate::types::Response {
    // curl -H "Content-Type: application/json" --data \
    //     '{comment: {text: "what kind of idiot name is foo?"},
    //        languages: ["en"],
    //        requestedAttributes: {TOXICITY:{}} }' \
    // https://commentanalyzer.googleapis.com/v1alpha1/comments:analyze?key=YOUR_KEY_HERE

    let url = format!("https://commentanalyzer.googleapis.com/v1alpha1/comments:analyze?key={}", api_key);

    let res = client.post(&url).json(&req).send().await?;

    let body = res.text().await?;

    let res = serde_json::from_str::<crate::types::RawApiResponse>(&body).map_err(|e| crate::types::ApiError::Json(e, body))?;

    res.extract()
}
