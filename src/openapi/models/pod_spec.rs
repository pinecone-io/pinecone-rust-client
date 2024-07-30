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

/// PodSpec : Configuration needed to deploy a pod-based index.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PodSpec {
    /// The environment where the index is hosted.
    #[serde(rename = "environment")]
    pub environment: String,
    /// The number of replicas. Replicas duplicate your index. They provide higher availability and throughput. Replicas can be scaled up or down as your needs change.
    #[serde(rename = "replicas")]
    pub replicas: i32,
    /// The number of shards. Shards split your data across multiple pods so you can fit more data into an index.
    #[serde(rename = "shards")]
    pub shards: i32,
    /// The type of pod to use. One of `s1`, `p1`, or `p2` appended with `.` and one of `x1`, `x2`, `x4`, or `x8`.
    #[serde(rename = "pod_type")]
    pub pod_type: String,
    /// The number of pods to be used in the index. This should be equal to `shards` x `replicas`.'
    #[serde(rename = "pods")]
    pub pods: i32,
    #[serde(rename = "metadata_config", skip_serializing_if = "Option::is_none")]
    pub metadata_config: Option<Box<models::PodSpecMetadataConfig>>,
    /// The name of the collection to be used as the source for the index.
    #[serde(rename = "source_collection", skip_serializing_if = "Option::is_none")]
    pub source_collection: Option<String>,
}

impl PodSpec {
    /// Configuration needed to deploy a pod-based index.
    pub fn new(environment: String, replicas: i32, shards: i32, pod_type: String, pods: i32) -> PodSpec {
        PodSpec {
            environment,
            replicas,
            shards,
            pod_type,
            pods,
            metadata_config: None,
            source_collection: None,
        }
    }
}

