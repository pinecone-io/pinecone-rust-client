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

/// IndexSpec : The spec object defines how the index should be deployed.  For serverless indexes, you define only the [cloud and region](http://docs.pinecone.io/guides/indexes/understanding-indexes#cloud-regions) where the index should be hosted. For pod-based indexes, you define the [environment](http://docs.pinecone.io/guides/indexes/understanding-indexes#pod-environments) where the index should be hosted, the [pod type and size](http://docs.pinecone.io/guides/indexes/understanding-indexes#pod-types) to use, and other index characteristics.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndexSpec {
    #[serde(rename = "serverless", skip_serializing_if = "Option::is_none")]
    pub serverless: Option<Box<models::ServerlessSpec>>,
    #[serde(rename = "pod", skip_serializing_if = "Option::is_none")]
    pub pod: Option<Box<models::PodSpec>>,
}

impl IndexSpec {
    /// The spec object defines how the index should be deployed.  For serverless indexes, you define only the [cloud and region](http://docs.pinecone.io/guides/indexes/understanding-indexes#cloud-regions) where the index should be hosted. For pod-based indexes, you define the [environment](http://docs.pinecone.io/guides/indexes/understanding-indexes#pod-environments) where the index should be hosted, the [pod type and size](http://docs.pinecone.io/guides/indexes/understanding-indexes#pod-types) to use, and other index characteristics.
    pub fn new() -> IndexSpec {
        IndexSpec {
            serverless: None,
            pod: None,
        }
    }
}
