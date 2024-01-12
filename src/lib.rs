mod types;
pub use types::*;

// throw a compilation error if both features are enabled
#[cfg(all(feature = "async", feature = "sync"))]
compile_error!("Cannot enable both async and sync features");

#[cfg(all(feature = "async", not(feature = "sync")))]
mod async_client;
#[cfg(all(feature = "async", not(feature = "sync")))]
mod client {
    pub use crate::async_client::Client;
}
#[cfg(all(feature = "async", not(feature = "sync")))]
pub use client::*;

// #[cfg(all(feature = "sync", not(feature = "async")))]
// mod sync_client;
// #[cfg(all(feature = "sync", not(feature = "async")))]
// mod client {
//     pub use crate::sync_client::Client;
// }
// #[cfg(all(not(feature = "sync"), not(feature = "async")))]
// pub use client::*;

#[derive(derive_builder::Builder, Clone, Debug)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct ClientConfig {
    pub api_key: String,
    #[builder(default = "16")]
    pub request_buffer_size: usize,
    #[builder(default = "16")]
    pub response_buffer_size: usize,
    #[builder(default = "128")]
    pub maximum_queue_size: usize,
    #[builder(default = "1100")]
    pub tick_rate: u64,
}

impl ClientConfigBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(0) = self.request_buffer_size {
            return Err("request buffer size cannot be 0".into());
        }

        if let Some(0) = self.response_buffer_size {
            return Err("response buffer size cannot be 0".into());
        }

        if let Some(0) = self.maximum_queue_size {
            return Err("maximum queue size cannot be 0".into());
        }

        if let Some(t) = self.tick_rate {
            if t < 1000 {
                return Err("tick rate cannot be less than 1000 ms".into());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_config_builder() {
        let config = ClientConfigBuilder::default()
            .api_key("api_key".into())
            .request_buffer_size(16)
            .response_buffer_size(16)
            .maximum_queue_size(128)
            .tick_rate(1100)
            .build()
            .unwrap();

        assert_eq!(config.api_key, "api_key");
        assert_eq!(config.request_buffer_size, 16);
        assert_eq!(config.response_buffer_size, 16);
        assert_eq!(config.maximum_queue_size, 128);
        assert_eq!(config.tick_rate, 1100);
    }

    #[test]
    fn test_client_config_builder_validate() {
        let config = ClientConfigBuilder::default()
            .api_key("api_key".into())
            .request_buffer_size(0)
            .response_buffer_size(0)
            .maximum_queue_size(0)
            .tick_rate(999)
            .build();

        assert!(config.is_err());
    }
}
