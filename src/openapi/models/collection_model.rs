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

/// CollectionModel : The CollectionModel describes the configuration and status of a Pinecone collection.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CollectionModel {
    /// The name of the collection.
    #[serde(rename = "name")]
    pub name: String,
    /// The size of the collection in bytes.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// The status of the collection.
    #[serde(rename = "status")]
    pub status: Status,
    /// The dimension of the vectors stored in each record held in the collection.
    #[serde(rename = "dimension", skip_serializing_if = "Option::is_none")]
    pub dimension: Option<i32>,
    /// The number of records stored in the collection.
    #[serde(rename = "vector_count", skip_serializing_if = "Option::is_none")]
    pub vector_count: Option<i32>,
    /// The environment where the collection is hosted.
    #[serde(rename = "environment")]
    pub environment: String,
}

impl CollectionModel {
    /// The CollectionModel describes the configuration and status of a Pinecone collection.
    pub fn new(name: String, status: Status, environment: String) -> CollectionModel {
        CollectionModel {
            name,
            size: None,
            status,
            dimension: None,
            vector_count: None,
            environment,
        }
    }
}
/// The status of the collection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Initializing")]
    Initializing,
    #[serde(rename = "Ready")]
    Ready,
    #[serde(rename = "Terminating")]
    Terminating,
}

impl Default for Status {
    fn default() -> Status {
        Self::Initializing
    }
}
