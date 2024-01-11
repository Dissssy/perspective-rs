#![allow(dead_code)]
mod request;

pub use request::Request;

pub type Response = Result<ApiResponse, ApiError>;

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum RawApiResponse {
    Valid(ApiResponse),
    Invalid(ApiErrorBody),
}

impl RawApiResponse {
    pub(crate) fn extract(self) -> Response {
        match self {
            RawApiResponse::Valid(r) => Ok(r),
            RawApiResponse::Invalid(e) => Err(ApiError::Api(e)),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct ApiResponse {}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("queue full")]
    QueueFull,
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    // maybe fancy error parsing later, for now though
    #[error("api error: {0}")]
    Api(ApiErrorBody),
    // #[error("API key not valid. Please pass a valid API key.")]
    // InvalidApiKey,
    // #[error("Quota exceeded")]
    // QuotaExceeded,
    // #[error("Comment must be non-empty.")]
    // CommentEmpty,
    // #[error("Comment text too long.")]
    // CommentTooLong,
    // #[error("Missing requested_attributes or Unknown requested attributes: {0:?}")]
    // MissingOrUnknownAttributes(Option<String>),
    // #[error("Attribute {0} does not support languages: {1:?}")]
    // LanguagesNotSupported(String, Vec<String>),
    // #[error("Unable to detect language")]
    // UnknownLanguage,
    // #[error("Context can have either entries or article_and_parent_comment, but both fields were populated.")]
    // InvalidContext,
    // #[error("Currently, only 'PLAIN_TEXT' comments are supported")]
    // UnsupportedCommentFormat,
    // #[error("Requested score type {0} is not supported by attribute {1}")]
    // UnsupportedScoreType(String, String),
}

// example error response
// {
//   "error": {
//     "code": 400,
//     "message": "API key not valid. Please pass a valid API key.",
//     "status": "INVALID_ARGUMENT",
//     "details": [
//       {
//         "@type": "type.googleapis.com/google.rpc.ErrorInfo",
//         "reason": "API_KEY_INVALID",
//         "domain": "googleapis.com",
//         "metadata": {
//           "service": "commentanalyzer.googleapis.com"
//         }
//       }
//     ]
//   }
// }

#[derive(serde::Deserialize, Debug)]
pub struct ApiErrorBody {
    error: ApiErrorBodyError,
}

impl std::fmt::Display for ApiErrorBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error.message)
    }
}

#[derive(serde::Deserialize, Debug)]
struct ApiErrorBodyError {
    code: u16,
    message: String,
    status: String,
}
