use super::{DeletionProtection, IndexModelSpec, IndexModelStatus, Metric, VectorType};
use crate::openapi::models::index_model::IndexModel as OpenApiIndexModel;
use crate::openapi::models::{CreateIndexForModelRequestEmbed, ModelIndexEmbed};
use serde::Serialize;
use std::collections::HashMap;

/// IndexModel : The IndexModel describes the configuration and status of a Pinecone index.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct IndexModel {
    /// Index name
    pub name: String,
    /// Index dimension
    pub dimension: Option<i32>,
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
    /// Index tags
    pub tags: Option<HashMap<String, String>>,
    /// Index embedding configuration
    pub embed: Option<ModelIndexEmbed>,
    /// Index vector type
    pub vector_type: VectorType,
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
            tags: openapi_index_model.tags,
            embed: openapi_index_model.embed.map(|emb| *emb),
            vector_type: openapi_index_model.vector_type,
        }
    }
}

/// A field mapping entry by type.
#[derive(Clone, Debug)]
pub enum FieldMapEntry {
    /// The name of the text field from your document model that is embedded.
    TextField(String),
}

/// A model parameter value of a specific type.
#[derive(Clone, Debug, Serialize)]
pub enum ModelParameterValue {
    /// A string value type
    StringVal(String),
    /// An integer value type
    IntVal(i32),
    /// A floating point value type
    FloatVal(f32),
    /// A boolean value type.
    BoolVal(bool),
}

/// Configuration options for the index with integrated embedding.
#[derive(Clone, Debug)]
pub struct CreateIndexForModelOptions {
    /// The name of the embedding model to use for the index.
    pub model: String,
    /// Identifies the name of the field from your document model that will be embedded. (Only one
    /// field is supported for now.)
    pub field_map: Vec<FieldMapEntry>,
    /// The distance metric to be used for similarity search. You can use 'euclidean', 'cosine', or 'dotproduct'. If not specified, the metric will be defaulted according to the model. Cannot be updated once set.
    pub metric: Option<Metric>,
    /// The desired vector dimension, if supported by the model.
    pub dimension: Option<i32>,
    /// The read parameters for the embedding model.
    pub read_parameters: Option<HashMap<String, ModelParameterValue>>,
    /// The write parameters for the embedding model.
    pub write_parameters: Option<HashMap<String, ModelParameterValue>>,
}

impl From<CreateIndexForModelOptions> for CreateIndexForModelRequestEmbed {
    fn from(options: CreateIndexForModelOptions) -> Self {
        let field_map = options
            .field_map
            .into_iter()
            .map(|entry| match entry {
                FieldMapEntry::TextField(field_name) => {
                    ("text", serde_json::Value::String(field_name))
                }
            })
            .collect();

        let read_parameters = options.read_parameters.map(|params| {
            params
                .into_iter()
                .map(|(key, value)| (key, serde_json::to_value(value).unwrap()))
                .collect()
        });

        let write_parameters = options.write_parameters.map(|params| {
            params
                .into_iter()
                .map(|(key, value)| (key, serde_json::to_value(value).unwrap()))
                .collect()
        });

        CreateIndexForModelRequestEmbed {
            model: options.model,
            field_map,
            metric: options.metric.map(|m| m.into()),
            read_parameters,
            write_parameters,
            dimension: options.dimension,
        }
    }
}
