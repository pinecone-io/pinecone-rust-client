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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigureIndexRequestSpecPod {
    /// The number of replicas. Replicas duplicate your index. They provide higher availability and throughput. Replicas can be scaled up or down as your needs change.
    #[serde(rename = "replicas", skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// The type of pod to use. One of `s1`, `p1`, or `p2` appended with `.` and one of `x1`, `x2`, `x4`, or `x8`.
    #[serde(rename = "pod_type", skip_serializing_if = "Option::is_none")]
    pub pod_type: Option<String>,
}

impl ConfigureIndexRequestSpecPod {
    pub fn new() -> ConfigureIndexRequestSpecPod {
        ConfigureIndexRequestSpecPod {
            replicas: None,
            pod_type: None,
        }
    }
}
