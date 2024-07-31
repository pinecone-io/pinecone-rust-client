use crate::openapi::models::Embedding as OpenApiEmbedding;

/// Embedding
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Embedding {
    /// Embedding values
    pub values: Vec<f32>,
}

impl From<OpenApiEmbedding> for Embedding {
    fn from(openapi_model: OpenApiEmbedding) -> Self {
        Embedding {
            values: openapi_model
                .values
                .unwrap_or_default()
                .into_iter()
                .map(|x| x as f32)
                .collect(),
        }
    }
}
