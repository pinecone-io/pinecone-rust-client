use openapi::models::create_index_request;
use openapi::models::serverless_spec;

/// Request parameters for creating an index.
#[derive(Debug, PartialEq)]
pub struct CreateIndexParams {
    /// The name of the index.
    /// Resource name must be 1-45 characters long, start and end with an alphanumeric character, and consist only of
    /// lower case alphanumeric characters or '-'.
    pub name: String,
    /// The dimensions of the vectors to be inserted in the index.
    pub dimension: u32,
    /// The distance metric to be used for similarity search. You can use 'euclidean', 'cosine', or 'dotproduct'.
    pub metric: Metric,
    /// The spec object defines how the index should be deployed.
    pub spec: Spec,
}

impl CreateIndexParams {
    /// Create a new instance of CreateIndexParams.
    pub fn new(
        name: &str,
        dimension: u32,
        metric: Option<Metric>,
        spec: Spec,
    ) -> CreateIndexParams {
        CreateIndexParams {
            name: name.to_string(),
            dimension,
            metric: metric.unwrap_or(Metric::Cosine),
            spec,
        }
    }
}

/// Spec for the index.
/// Can be either serverless or pod.
#[derive(Debug, PartialEq)]
pub enum Spec {
    /// Spec for a serverless index.
    Serverless {
        /// The public cloud where you would like your index hosted.
        cloud: Cloud,
        /// The region where you would like your index to be created.
        region: String,
    },
    /// Spec for a pod-based index.
    Pod {
        /// The environment where the index is hosted.
        environment: String,
        /// The number of replicas. Replicas duplicate your index. They provide higher availability and throughput.
        /// Replicas can be scaled up or down as your needs change.
        replicas: Option<i32>,
        /// The number of shards. Shards split your data across multiple pods so you can fit more data into an index.
        shards: Option<i32>,
        /// The type of pod to use. One of `s1`, `p1`, or `p2` appended with `.` and one of `x1`, `x2`, `x4`, or `x8`.
        pod_type: String,
        /// The number of pods to be used in the index. This should be equal to `shards` x `replicas`.
        pods: i32,
        /// Configuration for the behavior of Pinecone's internal metadata index. By default, all metadata is indexed;
        /// when `metadata_config` is present, only specified metadata fields are indexed.
        /// These configurations are only valid for use with pod-based indexes.
        metadata_config: Option<String>,
        /// The name of the collection to be used as the source for the index.
        source_collection: Option<String>,
    },
}

/// Distance metric to be used for similarity search.
#[derive(Debug, PartialEq)]
pub enum Metric {
    /// Cosine similarity
    Cosine,
    /// Euclidean distance
    Euclidean,
    /// Dot product
    Dotproduct,
}

impl Into<create_index_request::Metric> for Metric {
    fn into(self) -> create_index_request::Metric {
        match self {
            Metric::Cosine => create_index_request::Metric::Cosine,
            Metric::Euclidean => create_index_request::Metric::Euclidean,
            Metric::Dotproduct => create_index_request::Metric::Dotproduct,
        }
    }
}

/// Cloud where the index should be hosted.
#[derive(Debug, PartialEq)]
pub enum Cloud {
    /// Amazon Web Services
    Aws,
    /// Google Cloud Platform
    Gcp,
    /// Microsoft Azure
    Azure,
}

impl Into<serverless_spec::Cloud> for Cloud {
    fn into(self) -> serverless_spec::Cloud {
        match self {
            Cloud::Aws => serverless_spec::Cloud::Aws,
            Cloud::Gcp => serverless_spec::Cloud::Gcp,
            Cloud::Azure => serverless_spec::Cloud::Azure,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_create_index_params() {
        let create_index_params = CreateIndexParams::new(
            "test_index",
            10,
            None,
            Spec::Serverless {
                cloud: Cloud::Aws,
                region: "us-west-2".to_string(),
            },
        );

        assert_eq!(create_index_params.name, "test_index");
        assert_eq!(create_index_params.dimension, 10);
        assert_eq!(create_index_params.metric, Metric::Cosine);
        assert_eq!(
            create_index_params.spec,
            Spec::Serverless {
                cloud: Cloud::Aws,
                region: "us-west-2".to_string(),
            }
        );
    }
}
