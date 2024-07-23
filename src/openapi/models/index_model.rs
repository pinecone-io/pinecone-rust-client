/*
 * Pinecone Control Plane API
 *
 * Pinecone is a vector database that makes it easy to search and retrieve billions of high-dimensional vectors.
 *
 * The version of the OpenAPI document: 2024-07
 * Contact: support@pinecone.io
 * Generated by: https://openapi-generator.tech
 */

use crate::openapi::models;
use serde::{Deserialize, Serialize};

/// IndexModel : The IndexModel describes the configuration and status of a Pinecone index.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndexModel {
    /// The name of the index. Resource name must be 1-45 characters long, start and end with an alphanumeric character, and consist only of lower case alphanumeric characters or '-'. 
    #[serde(rename = "name")]
    pub name: String,
    /// The dimensions of the vectors to be inserted in the index.
    #[serde(rename = "dimension")]
    pub dimension: i32,
    /// The distance metric to be used for similarity search. You can use 'euclidean', 'cosine', or 'dotproduct'.
    #[serde(rename = "metric")]
    pub metric: Metric,
    /// The URL address where the index is hosted.
    #[serde(rename = "host")]
    pub host: String,
    #[serde(rename = "spec")]
    pub spec: Box<models::IndexModelSpec>,
    #[serde(rename = "status")]
    pub status: Box<models::IndexModelStatus>,
}

impl IndexModel {
    /// The IndexModel describes the configuration and status of a Pinecone index.
    pub fn new(name: String, dimension: i32, metric: Metric, host: String, spec: models::IndexModelSpec, status: models::IndexModelStatus) -> IndexModel {
        IndexModel {
            name,
            dimension,
            metric,
            host,
            spec: Box::new(spec),
            status: Box::new(status),
        }
    }
}
/// The distance metric to be used for similarity search. You can use 'euclidean', 'cosine', or 'dotproduct'.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Metric {
    #[serde(rename = "cosine")]
    Cosine,
    #[serde(rename = "euclidean")]
    Euclidean,
    #[serde(rename = "dotproduct")]
    Dotproduct,
}

impl Default for Metric {
    fn default() -> Metric {
        Self::Cosine
    }
}
