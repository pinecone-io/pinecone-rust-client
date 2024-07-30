use crate::models::{DeletionProtection, IndexModelSpec, IndexModelStatus, Metric};
use crate::openapi::models::index_model::IndexModel as OpenApiIndexModel;

#[derive(Clone, Debug, PartialEq)]
pub struct IndexModel {
    pub name: String,
    pub dimension: i32,
    pub metric: Metric,
    pub host: String,
    pub deletion_protection: Option<DeletionProtection>,
    pub spec: Box<IndexModelSpec>,
    pub status: Box<IndexModelStatus>,
}

impl From<OpenApiIndexModel> for IndexModel {
    fn from(openapi_index_model: OpenApiIndexModel) -> Self {
        IndexModel {
            name: openapi_index_model.name,
            dimension: openapi_index_model.dimension,
            metric: openapi_index_model.metric.into(),
            host: openapi_index_model.host,
            deletion_protection: openapi_index_model.deletion_protection,
            spec: openapi_index_model.spec,
            status: openapi_index_model.status,
        }
    }
}

impl IndexModel {
    pub fn new(
        name: String,
        dimension: i32,
        metric: Metric,
        host: String,
        spec: IndexModelSpec,
        status: IndexModelStatus,
    ) -> IndexModel {
        IndexModel {
            name,
            dimension,
            metric,
            host,
            deletion_protection: None,
            spec: Box::new(spec),
            status: Box::new(status),
        }
    }
}
