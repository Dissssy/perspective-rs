#![allow(dead_code)]
mod request;
mod response;

use std::fmt::Display;

pub use request::*;
pub use response::*;

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum RawApiResponse {
    Valid(ApiResponse),
    ValidNoResponse(EmptyApiResponse),
    Invalid(ApiErrorBody),
}

impl RawApiResponse {
    pub(crate) fn extract(self) -> Response {
        match self {
            RawApiResponse::Valid(r) => Ok(r),
            RawApiResponse::ValidNoResponse(r) => Err(ApiError::EmptyResponse(r)),
            RawApiResponse::Invalid(e) => Err(ApiError::Api(e)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("receiver has been taken")]
    ReceiverTaken,
    #[error("queue full")]
    QueueFull,
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("json error: {0}\n{1:#?}")]
    Json(serde_json::Error, String),
    #[error("empty response: {0:#?}")]
    EmptyResponse(EmptyApiResponse),

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

/// The attribute types that the API can return.
/// Supported by every language available with the highest accuracy:
/// - `Toxicity`
/// - `SevereToxicity`
/// - `IdentityAttack`
/// - `Insult`
/// - `Profanity`
/// - `Threat`
///
/// Experimental, language support may vary, not as accurate:
/// - `ToxicityExperimental`
/// - `SevereToxicityExperimental`
/// - `IdentityAttackExperimental`
/// - `InsultExperimental`
/// - `ProfanityExperimental`
/// - `ThreatExperimental`
/// - `SexuallyExplicit`
/// - `Flirtation`
///
/// New York Times attributes, trained only on NYT comments so accuracy may vary, only supported in English:
/// - `AttackOnAuthor`
/// - `AttackOnCommenter`
/// - `Incoherent`
/// - `Inflammatory`
/// - `LikelyToReject`
/// - `Obscene`
/// - `Spam`
/// - `Unsubstantial`
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Attribute {
    /// A rude, disrespectful, or unreasonable comment that is likely to make people leave a discussion.
    #[serde(rename = "TOXICITY")]
    Toxicity,
    /// A very hateful, aggressive, disrespectful comment or otherwise very likely to make a user leave a discussion or give up on sharing their perspective. This attribute is much less sensitive to more mild forms of toxicity, such as comments that include positive uses of curse words.
    #[serde(rename = "SEVERE_TOXICITY")]
    SevereToxicity,
    /// Negative or hateful comments targeting someone because of their identity.
    #[serde(rename = "IDENTITY_ATTACK")]
    IdentityAttack,
    /// Insulting, inflammatory, or negative comment towards a person or a group of people.
    #[serde(rename = "INSULT")]
    Insult,
    /// Swear words, curse words, or other obscene or profane language.
    #[serde(rename = "PROFANITY")]
    Profanity,
    /// Describes an intention to inflict pain, injury, or violence against an individual or group.
    #[serde(rename = "THREAT")]
    Threat,
    /// A rude, disrespectful, or unreasonable comment that is likely to make people leave a discussion.
    #[serde(rename = "TOXICITY_EXPERIMENTAL")]
    ToxicityExperimental,
    /// A very hateful, aggressive, disrespectful comment or otherwise very likely to make a user leave a discussion or give up on sharing their perspective. This attribute is much less sensitive to more mild forms of toxicity, such as comments that include positive uses of curse words.
    #[serde(rename = "SEVERE_TOXICITY_EXPERIMENTAL")]
    SevereToxicityExperimental,
    /// Negative or hateful comments targeting someone because of their identity.
    #[serde(rename = "IDENTITY_ATTACK_EXPERIMENTAL")]
    IdentityAttackExperimental,
    /// Insulting, inflammatory, or negative comment towards a person or a group of people.
    #[serde(rename = "INSULT_EXPERIMENTAL")]
    InsultExperimental,
    /// Swear words, curse words, or other obscene or profane language.
    #[serde(rename = "PROFANITY_EXPERIMENTAL")]
    ProfanityExperimental,
    /// Describes an intention to inflict pain, injury, or violence against an individual or group.
    #[serde(rename = "THREAT_EXPERIMENTAL")]
    ThreatExperimental,
    /// Contains references to sexual acts, body parts, or other lewd content.
    #[serde(rename = "SEXUALLY_EXPLICIT")]
    SexuallyExplicit,
    /// Pickup lines, complimenting appearance, subtle sexual innuendos, etc.
    #[serde(rename = "FLIRTATION")]
    Flirtation,
    /// Attack on the author of an article or post.
    #[serde(rename = "ATTACK_ON_AUTHOR")]
    AttackOnAuthor,
    /// Attack on fellow commenter.
    #[serde(rename = "ATTACK_ON_COMMENTER")]
    AttackOnCommenter,
    /// Difficult to understand, nonsensical.
    #[serde(rename = "INCOHERENT")]
    Incoherent,
    /// Intending to provoke or inflame.
    #[serde(rename = "INFLAMMATORY")]
    Inflammatory,
    /// Overall measure of the likelihood for the comment to be rejected according to the NYT's moderation.
    #[serde(rename = "LIKELY_TO_REJECT")]
    LikelyToReject,
    /// Obscene or vulgar language such as cursing.
    #[serde(rename = "OBSCENE")]
    Obscene,
    /// Irrelevant and unsolicited commercial content.
    #[serde(rename = "SPAM")]
    Spam,
    /// Trivial or short comments.
    #[serde(rename = "UNSUBSTANTIAL")]
    Unsubstantial,
}

impl Attribute {
    pub(crate) fn all() -> [Attribute; 22] {
        [
            Attribute::Toxicity,
            Attribute::SevereToxicity,
            Attribute::IdentityAttack,
            Attribute::Insult,
            Attribute::Profanity,
            Attribute::Threat,
            Attribute::ToxicityExperimental,
            Attribute::SevereToxicityExperimental,
            Attribute::IdentityAttackExperimental,
            Attribute::InsultExperimental,
            Attribute::ProfanityExperimental,
            Attribute::ThreatExperimental,
            Attribute::SexuallyExplicit,
            Attribute::Flirtation,
            Attribute::AttackOnAuthor,
            Attribute::AttackOnCommenter,
            Attribute::Incoherent,
            Attribute::Inflammatory,
            Attribute::LikelyToReject,
            Attribute::Obscene,
            Attribute::Spam,
            Attribute::Unsubstantial,
        ]
    }

    pub(crate) fn all_normal() -> [Attribute; 6] {
        [
            Attribute::Toxicity,
            Attribute::SevereToxicity,
            Attribute::IdentityAttack,
            Attribute::Insult,
            Attribute::Profanity,
            Attribute::Threat,
        ]
    }

    pub(crate) fn all_experimental() -> [Attribute; 10] {
        [
            Attribute::ToxicityExperimental,
            Attribute::SevereToxicityExperimental,
            Attribute::IdentityAttackExperimental,
            Attribute::InsultExperimental,
            Attribute::ProfanityExperimental,
            Attribute::ThreatExperimental,
            Attribute::SexuallyExplicit,
            Attribute::Flirtation,
            Attribute::AttackOnAuthor,
            Attribute::AttackOnCommenter,
        ]
    }

    pub(crate) fn all_nyt() -> [Attribute; 6] {
        [
            Attribute::Incoherent,
            Attribute::Inflammatory,
            Attribute::LikelyToReject,
            Attribute::Obscene,
            Attribute::Spam,
            Attribute::Unsubstantial,
        ]
    }

    pub fn check_compatibility(&self, lang: &LanguageCode) -> AttributeCompatibility {
        match *self {
            _ if lang == &LanguageCode::English => AttributeCompatibility::Compatible,
            Attribute::Toxicity => AttributeCompatibility::Compatible,
            Attribute::SevereToxicity => AttributeCompatibility::Compatible,
            Attribute::IdentityAttack => AttributeCompatibility::Compatible,
            Attribute::Insult => AttributeCompatibility::Compatible,
            Attribute::Profanity => AttributeCompatibility::Compatible,
            Attribute::Threat => AttributeCompatibility::Compatible,
            Attribute::ToxicityExperimental => AttributeCompatibility::Unknown,
            Attribute::SevereToxicityExperimental => AttributeCompatibility::Unknown,
            Attribute::IdentityAttackExperimental => AttributeCompatibility::Unknown,
            Attribute::InsultExperimental => AttributeCompatibility::Unknown,
            Attribute::ProfanityExperimental => AttributeCompatibility::Unknown,
            Attribute::ThreatExperimental => AttributeCompatibility::Unknown,
            Attribute::SexuallyExplicit => AttributeCompatibility::Unknown,
            Attribute::Flirtation => AttributeCompatibility::Unknown,
            Attribute::AttackOnAuthor => AttributeCompatibility::Incompatible,
            Attribute::AttackOnCommenter => AttributeCompatibility::Incompatible,
            Attribute::Incoherent => AttributeCompatibility::Incompatible,
            Attribute::Inflammatory => AttributeCompatibility::Incompatible,
            Attribute::LikelyToReject => AttributeCompatibility::Incompatible,
            Attribute::Obscene => AttributeCompatibility::Incompatible,
            Attribute::Spam => AttributeCompatibility::Incompatible,
            Attribute::Unsubstantial => AttributeCompatibility::Incompatible,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeCompatibility {
    Compatible,
    Unknown,
    Incompatible,
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::Toxicity => write!(f, "TOXICITY"),
            Attribute::SevereToxicity => write!(f, "SEVERE_TOXICITY"),
            Attribute::IdentityAttack => write!(f, "IDENTITY_ATTACK"),
            Attribute::Insult => write!(f, "INSULT"),
            Attribute::Profanity => write!(f, "PROFANITY"),
            Attribute::Threat => write!(f, "THREAT"),
            Attribute::ToxicityExperimental => write!(f, "TOXICITY_EXPERIMENTAL"),
            Attribute::SevereToxicityExperimental => write!(f, "SEVERE_TOXICITY_EXPERIMENTAL"),
            Attribute::IdentityAttackExperimental => write!(f, "IDENTITY_ATTACK_EXPERIMENTAL"),
            Attribute::InsultExperimental => write!(f, "INSULT_EXPERIMENTAL"),
            Attribute::ProfanityExperimental => write!(f, "PROFANITY_EXPERIMENTAL"),
            Attribute::ThreatExperimental => write!(f, "THREAT_EXPERIMENTAL"),
            Attribute::SexuallyExplicit => write!(f, "SEXUALLY_EXPLICIT"),
            Attribute::Flirtation => write!(f, "FLIRTATION"),
            Attribute::AttackOnAuthor => write!(f, "ATTACK_ON_AUTHOR"),
            Attribute::AttackOnCommenter => write!(f, "ATTACK_ON_COMMENTER"),
            Attribute::Incoherent => write!(f, "INCOHERENT"),
            Attribute::Inflammatory => write!(f, "INFLAMMATORY"),
            Attribute::LikelyToReject => write!(f, "LIKELY_TO_REJECT"),
            Attribute::Obscene => write!(f, "OBSCENE"),
            Attribute::Spam => write!(f, "SPAM"),
            Attribute::Unsubstantial => write!(f, "UNSUBSTANTIAL"),
        }
    }
}
