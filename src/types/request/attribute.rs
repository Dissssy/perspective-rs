#[derive(serde::Serialize, derive_builder::Builder, Clone, Debug)]
pub struct AttributeOptions {
    /// The score type returned for this attribute. Currently, only "PROBABILITY" is supported. Probability scores are in the range [0,1].
    #[builder(setter(into), default = "ScoreType::default()")]
    pub(crate) score_type: ScoreType,
    /// The API won't return scores that are below this threshold for this attribute. By default, all scores are returned.
    #[builder(default, setter(strip_option))]
    pub(crate) score_threshold: Option<f64>,
}

impl Default for AttributeOptions {
    fn default() -> Self {
        Self {
            score_type: ScoreType::default(),
            score_threshold: Some(0.0),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum ScoreType {
    #[serde(rename = "PROBABILITY")]
    Probability,
}

impl Default for ScoreType {
    fn default() -> Self {
        Self::Probability
    }
}
