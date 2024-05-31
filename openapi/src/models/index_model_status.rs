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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndexModelStatus {
    #[serde(rename = "ready")]
    pub ready: bool,
    #[serde(rename = "state")]
    pub state: State,
}

impl IndexModelStatus {
    pub fn new(ready: bool, state: State) -> IndexModelStatus {
        IndexModelStatus {
            ready,
            state,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "Initializing")]
    Initializing,
    #[serde(rename = "InitializationFailed")]
    InitializationFailed,
    #[serde(rename = "ScalingUp")]
    ScalingUp,
    #[serde(rename = "ScalingDown")]
    ScalingDown,
    #[serde(rename = "ScalingUpPodSize")]
    ScalingUpPodSize,
    #[serde(rename = "ScalingDownPodSize")]
    ScalingDownPodSize,
    #[serde(rename = "Terminating")]
    Terminating,
    #[serde(rename = "Ready")]
    Ready,
}

impl Default for State {
    fn default() -> State {
        Self::Initializing
    }
}

