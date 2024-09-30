use super::{DeletionProtection, IndexModelSpec, IndexModelStatus, Metric};
use crate::openapi::models::index_model::IndexModel as OpenApiIndexModel;

/// IndexModel : The IndexModel describes the configuration and status of a Pinecone index.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct IndexModel {
    /// Index name
    pub name: String,
    /// Index dimension
    pub dimension: i32,
    /// Index metric
    pub metric: Metric,
    /// Index host
    pub host: String,
    /// Index deletion protection configuration
    pub deletion_protection: Option<DeletionProtection>,
    /// Index specs
    pub spec: IndexModelSpec,
    /// Index model specs
    pub status: IndexModelStatus,
}

impl From<OpenApiIndexModel> for IndexModel {
    fn from(openapi_index_model: OpenApiIndexModel) -> Self {
        IndexModel {
            name: openapi_index_model.name,
            dimension: openapi_index_model.dimension,
            metric: openapi_index_model.metric.into(),
            host: openapi_index_model.host,
            deletion_protection: openapi_index_model.deletion_protection,
            spec: *openapi_index_model.spec,
            status: *openapi_index_model.status,
        }
    }
}
