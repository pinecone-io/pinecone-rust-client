use openapi::models::create_index_request;
use openapi::models::serverless_spec;

#[derive(Debug, PartialEq)]
pub struct CreateIndexParams {
    pub name: String,
    pub dimension: u32,
    pub metric: Metric,
    pub spec: Spec,
}

impl CreateIndexParams {
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

#[derive(Debug, PartialEq)]
pub enum Spec {
    Serverless {
        cloud: Cloud,
        region: String,
    },
    Pod {
        environment: String,
        replicas: Option<i32>,
        shards: Option<i32>,
        pod_type: String,
        pods: i32,
        metadata_config: Option<String>,
        source_collection: Option<String>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Metric {
    Cosine,
    Euclidean,
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

#[derive(Debug, PartialEq)]
pub enum Cloud {
    Aws,
    Gcp,
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
