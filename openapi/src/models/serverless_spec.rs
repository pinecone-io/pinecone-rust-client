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

/// ServerlessSpec : Configuration needed to deploy a serverless index.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerlessSpec {
    /// The public cloud where you would like your index hosted.
    #[serde(rename = "cloud")]
    pub cloud: Cloud,
    /// The region where you would like your index to be created. 
    #[serde(rename = "region")]
    pub region: String,
}

impl ServerlessSpec {
    /// Configuration needed to deploy a serverless index.
    pub fn new(cloud: Cloud, region: String) -> ServerlessSpec {
        ServerlessSpec {
            cloud,
            region,
        }
    }
}
/// The public cloud where you would like your index hosted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Cloud {
    #[serde(rename = "gcp")]
    Gcp,
    #[serde(rename = "aws")]
    Aws,
    #[serde(rename = "azure")]
    Azure,
}

impl Default for Cloud {
    fn default() -> Cloud {
        Self::Gcp
    }
}

