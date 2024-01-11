#[derive(serde::Serialize, derive_builder::Builder, Clone, Debug)]
pub struct AttributeOptions {
    /// The score type returned for this attribute. Currently, only "PROBABILITY" is supported. Probability scores are in the range [0,1].
    #[builder(setter(into), default = "ScoreType::default()")]
    pub(crate) score_type: ScoreType,
    /// The API won't return scores that are below this threshold for this attribute. By default, all scores are returned.
    #[builder(default)]
    pub(crate) score_threshold: Option<f64>,
}

#[derive(serde::Serialize, Clone, Debug)]
pub enum ScoreType {
    #[serde(rename = "PROBABILITY")]
    Probability,
}

impl Default for ScoreType {
    fn default() -> Self {
        Self::Probability
    }
}

/// The attribute types that the API can return.
/// Supported by every language available with the highest accuracy:
/// - `Toxicity`
/// - `SevereToxicity`
/// - `IdentityAttack`
/// - `Insult`
/// - `Profanity`
/// - `Threat`
/// Experimental, language support may vary, not as accurate:
/// - `ToxicityExperimental`
/// - `SevereToxicityExperimental`
/// - `IdentityAttackExperimental`
/// - `InsultExperimental`
/// - `ProfanityExperimental`
/// - `ThreatExperimental`
/// - `SexuallyExplicit`
/// - `Flirtation`
/// New York Times attributes, trained only on NYT comments so accuracy may vary, only supported in English:
/// - `AttackOnAuthor`
/// - `AttackOnCommenter`
/// - `Incoherent`
/// - `Inflammatory`
/// - `LikelyToReject`
/// - `Obscene`
/// - `Spam`
/// - `Unsubstantial`
#[derive(serde::Serialize, Clone, Debug)]
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
