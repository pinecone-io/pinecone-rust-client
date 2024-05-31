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

/// ListIndexes401Response : The response shape used for all error responses.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListIndexes401Response {
    /// The HTTP status code of the error.
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "error")]
    pub error: Box<models::ListIndexes401ResponseError>,
}

impl ListIndexes401Response {
    /// The response shape used for all error responses.
    pub fn new(status: i32, error: models::ListIndexes401ResponseError) -> ListIndexes401Response {
        ListIndexes401Response {
            status,
            error: Box::new(error),
        }
    }
}

