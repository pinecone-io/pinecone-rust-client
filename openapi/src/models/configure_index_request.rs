/*
 * Pinecone Control Plane API
 *
 * Pinecone is a vector database that makes it easy to search and retrieve billions of high-dimensional vectors.
 *
 * The version of the OpenAPI document: 2024-07
 * Contact: support@pinecone.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ConfigureIndexRequest : Configuration used to scale an index.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigureIndexRequest {
    #[serde(rename = "spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<models::ConfigureIndexRequestSpec>>,
    #[serde(rename = "deletion_protection", skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<models::DeletionProtection>,
}

impl ConfigureIndexRequest {
    /// Configuration used to scale an index.
    pub fn new() -> ConfigureIndexRequest {
        ConfigureIndexRequest {
            spec: None,
            deletion_protection: None,
        }
    }
}

