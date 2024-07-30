use super::EmbeddingsListUsage;

/// EmbeddingsList : Embeddings generated for the input
#[derive(Clone, Default, Debug, PartialEq)]
pub struct EmbeddingsList {
    /// The model used to generate the embeddings.
    pub model: String,
    /// The embeddings generated by the model.
    pub data: Vec<Vec<f32>>,
    /// The total number of tokens processed.
    pub usage: EmbeddingsListUsage,
}

impl From<crate::openapi::models::EmbeddingsList> for EmbeddingsList {
    fn from(openapi_model: crate::openapi::models::EmbeddingsList) -> Self {
        EmbeddingsList {
            model: openapi_model.model.unwrap_or_default(),
            data: openapi_model
                .data
                .unwrap_or_default()
                .into_iter()
                .map(|x| x.into())
                .collect(),
            usage: (*openapi_model.usage.unwrap_or_default()).into(),
        }
    }
}

impl From<crate::openapi::models::Embedding> for Vec<f32> {
    fn from(openapi_model: crate::openapi::models::Embedding) -> Self {
        openapi_model
            .values
            .unwrap_or_default()
            .clone()
            .into_iter()
            .map(|x| x as f32)
            .collect()
    }
}