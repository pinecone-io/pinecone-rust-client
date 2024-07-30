/// EmbeddingsListUsage : Usage statistics for model inference including any instruction prefixes
#[derive(Clone, Default, Debug, PartialEq)]
pub struct EmbeddingsListUsage {
    /// The total number of tokens processed.
    pub total_tokens: i32,
}

impl From<crate::openapi::models::EmbeddingsListUsage> for EmbeddingsListUsage {
    fn from(openapi_model: crate::openapi::models::EmbeddingsListUsage) -> Self {
        EmbeddingsListUsage {
            total_tokens: openapi_model.total_tokens.unwrap_or(0),
        }
    }
}
