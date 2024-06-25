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

/// Embedding : Embedding of a single input
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Embedding {
    /// The embedding values.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<f64>>,
}

impl Embedding {
    /// Embedding of a single input
    pub fn new() -> Embedding {
        Embedding {
            values: None,
        }
    }
}

