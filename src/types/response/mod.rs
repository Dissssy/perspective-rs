pub type Response = Result<ApiResponse, super::ApiError>;

// {
//   "attributeScores": {
//     string: {
//       "summaryScore": {
//         "value": float,
//         "type": string
//       }
//       "spanScores": [{
//         "begin": int,
//         "end": int,
//         "score": {
//            "value": float,
//            "type": string
//         }
//       }]
//     }
//   },
//   "languages": [string],
//   "clientToken": string
// }

// attributeScores
// A map from attribute name to per-attribute score objects. The attribute names will mirror the request's requestedAttributes.

// attributeScores[name].summaryScore.value
// The attribute summary score for the entire comment. All attributes will return a summaryScore (unless the request specified a scoreThreshold for the attribute that the summaryScore did not exceed).

// attributeScores[name].summaryScore.type
// This mirrors the requested scoreType for this attribute.

// attributeScores[name].spanScores
// A list of per-span scores for this attribute. These scores apply to different parts of the request's comment.text. Note: Some attributes may not return spanScores at all.

// attributeScores[name].spanScores[].begin
// Beginning character index of the text span in the request comment.

// attributeScores[name].spanScores[].end
// End of the text span in the request comment.

// attributeScores[name].spanScores[].score.value
// The attribute score for the span delimited by begin and end.

// attributeScores[name].spanScores[].score.type
// Same as summaryScore.type.

// languages
// Mirrors the request's languages. If no languages were specified, the API returns the auto-detected language.

// clientToken
// Mirrors the request's clientToken.

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ApiResponse {
    #[serde(rename = "attributeScores")]
    pub attribute_scores: std::collections::HashMap<super::Attribute, AttributeScores>,
    pub languages: Vec<super::LanguageCode>,
    #[serde(rename = "clientToken")]
    pub client_token: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct AttributeScores {
    #[serde(rename = "summaryScore")]
    pub summary_score: Score,
    #[serde(rename = "spanScores")]
    pub span_scores: Option<Vec<SpanScore>>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Score {
    pub value: f64,
    #[serde(rename = "type")]
    pub score_type: super::ScoreType,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct SpanScore {
    pub begin: usize,
    pub end: usize,
    pub score: Score,
}

// {
//   \"languages\": [
//     \"en\"
//   ],
//   \"clientToken\": \"eJwBTACz/y8DaYI/kqHB9/wx2vys6RB7wZy2+a0gjvf5sA0/k42kTV8ttMlDmKaounSmov21o73xkLX9oSCMPcC9DzyTjKZMWyuzDXqQoa25cqd/gyqP\",
//   \"detectedLanguages\": [
//     \"en\"
//   ]
// }

#[derive(serde::Deserialize, Debug, Clone)]
pub struct EmptyApiResponse {
    pub languages: Vec<super::LanguageCode>,
    #[serde(rename = "clientToken")]
    pub client_token: Option<String>,
}
