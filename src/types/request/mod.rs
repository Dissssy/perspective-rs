mod attribute;
use crate::AttributeCompatibility;

use super::Attribute;
pub use attribute::*;
use std::collections::HashMap;

/// The request object for the `analyze` method.
#[derive(serde::Serialize, derive_builder::Builder, Clone, Debug)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct Request {
    /// The comment data to analyze.
    #[builder(setter(into))]
    pub(crate) comment: Comment,
    /// The context of the comment.
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) context: Option<Context>,
    /// The requested attributes.
    #[builder(setter(into))]
    #[serde(rename = "requestedAttributes")]
    pub(crate) requested_attributes: HashMap<Attribute, AttributeOptions>,
    /// A boolean value that indicates if the request should return spans that describe the scores for each part of the text (currently done at per-sentence level). Defaults to false.
    #[builder(default, setter(into, strip_option))]
    #[serde(rename = "spanAnnotations", skip_serializing_if = "Option::is_none")]
    pub(crate) span_annotations: Option<bool>,
    /// A list of ISO 631-1 two-letter language codes specifying the language(s) that comment is in (for example, "en", "es", "fr", "de", etc). If unspecified, the API will auto-detect the comment language. If language detection fails, the API returns an error. Note: See currently supported languages on the ‘Attributes and Languages’ page. There is no simple way to use the API across languages with production support and languages with experimental support only.
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) languages: Option<Vec<LanguageCode>>,
    /// Whether the API is permitted to store comment and context from this request. Stored comments will be used for future research and community attribute building purposes to improve the API over time. Defaults to false (request data may be stored). Warning: This should be set to true if data being submitted is private (i.e. not publicly accessible), or if the data submitted contains content written by someone under 13 years old (or the relevant age determined by applicable law in your jurisdiction).
    #[builder(default, setter(strip_option))]
    #[serde(rename = "doNotStore", skip_serializing_if = "Option::is_none")]
    pub(crate) do_not_store: Option<bool>,
    /// An opaque token that is echoed back in the response.
    #[builder(setter(into), default)]
    #[serde(rename = "clientToken", skip_serializing_if = "Option::is_none")]
    pub(crate) client_token: Option<String>,
    /// An opaque session ID. This should be set for authorship experiences by the client side so that groups of requests can be grouped together into a session. This should not be used for any user-specific id. This is intended for abuse protection and individual sessions of interaction.
    #[builder(setter(into), default)]
    #[serde(rename = "sessionId", skip_serializing_if = "Option::is_none")]
    pub(crate) session_id: Option<String>,
    /// An opaque identifier associating this comment with a particular community within your platform. If set, this field allows us to differentiate comments from different communities, as each community may have different norms.
    #[builder(setter(into), default)]
    #[serde(rename = "communityId", skip_serializing_if = "Option::is_none")]
    pub(crate) community_id: Option<String>,
}

impl RequestBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(attr) = self.requested_attributes.as_ref() {
            if attr.is_empty() {
                return Err("requested attributes cannot be empty".into());
            }
            if let Some(Some(langs)) = self.languages.as_ref() {
                if attr.keys().any(|a| langs.iter().any(|l| a.check_compatibility(l) == AttributeCompatibility::Incompatible)) {
                    return Err("requested attributes are incompatible with the selected language(s)".into());
                }
            }
        }
        Ok(())
    }

    /// Add an attribute to the request.
    pub fn add_attribute(&mut self, attribute: Attribute, options: AttributeOptions) -> &mut Self {
        match self.requested_attributes {
            Some(ref mut map) => {
                map.insert(attribute, options);
            }
            None => {
                let mut map = HashMap::new();
                map.insert(attribute, options);
                self.requested_attributes = Some(map);
            }
        }

        self
    }

    pub fn all_attributes(&mut self) -> &mut Self {
        self.requested_attributes = Some(Attribute::all().into_iter().map(|a| (a, AttributeOptions::default())).collect());
        self
    }
}

/// The comment data to analyze.
#[derive(serde::Serialize, derive_builder::Builder, Clone, Debug)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct Comment {
    /// The text to score. This is assumed to be utf8 raw text of the text to be checked. Emoji and other non-ascii characters can be included (HTML will probably result in lower performance).
    #[builder(setter(into))]
    pub(crate) text: String,
    /// The text type of comment.text. Either "PLAIN_TEXT" or "HTML". Currently only "PLAIN_TEXT" is supported.
    #[serde(rename = "type")]
    #[builder(setter(into, strip_option), default)]
    pub(crate) type_: Option<TextType>,
}

impl<T> From<T> for Comment
where
    T: ToString,
{
    fn from(s: T) -> Self {
        Self { text: s.to_string(), type_: None }
    }
}

impl CommentBuilder {
    fn validate(&self) -> Result<(), String> {
        // ensure text does not exceed 20kb
        if self.text.as_ref().map(|s| s.bytes().len()).unwrap_or(0) > 20_000 {
            return Err("comment text cannot exceed 20kb".into());
        }

        Ok(())
    }
}

/// The context of the comment.
#[derive(serde::Serialize, derive_builder::Builder, Clone, Debug)]
pub struct Context {
    /// A list of objects providing the context for comment. The API currently does not make use of this field, but it may influence API responses in the future.
    #[builder(setter(into))]
    pub(crate) entries: Vec<Entry>,
}

/// A context object.
#[derive(serde::Serialize, derive_builder::Builder, Clone, Debug)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct Entry {
    /// The text of a context object. The maximum size of context entry is 1MB.
    #[builder(setter(into))]
    pub(crate) text: String,
    /// The text type of the corresponding context text. Same type as comment.text. Currently only "PLAIN TEXT" is supported.
    #[serde(rename = "type")]
    #[builder(setter(into, strip_option), default)]
    pub(crate) type_: Option<TextType>,
}

impl EntryBuilder {
    fn validate(&self) -> Result<(), String> {
        // ensure text does not exceed 1MB
        if self.text.as_ref().map(|s| s.bytes().len()).unwrap_or(0) > 1_000_000 {
            return Err("context entry text cannot exceed 1MB".into());
        }

        Ok(())
    }
}

/// The text type, either plain text or HTML. Currently only plain text is supported.
#[derive(serde::Serialize, Clone, Debug)]
pub enum TextType {
    #[serde(rename = "PLAIN_TEXT")]
    PlainText,
    #[serde(rename = "HTML")]
    Html,
}

impl Default for TextType {
    fn default() -> Self {
        Self::PlainText
    }
}

/// The language codes that the API can accept.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LanguageCode {
    #[serde(rename = "ar")]
    Arabic,
    #[serde(rename = "zh")]
    Chinese,
    #[serde(rename = "cs")]
    Czech,
    #[serde(rename = "nl")]
    Dutch,
    #[serde(rename = "en")]
    English,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "hi")]
    Hindi,
    #[serde(rename = "hi-Latn")]
    Hinglish,
    #[serde(rename = "id")]
    Indonesian,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "pl")]
    Polish,
    #[serde(rename = "pt")]
    Portuguese,
    #[serde(rename = "ru")]
    Russian,
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "sv")]
    Swedish,
    #[serde(deserialize_with = "deserialize_unknown")]
    Other(String),
}

fn deserialize_unknown<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = <String as serde::Deserialize>::deserialize(deserializer)?;
    Ok(s)
}
