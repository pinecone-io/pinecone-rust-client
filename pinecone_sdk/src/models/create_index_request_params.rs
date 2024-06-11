
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

#[derive(Debug, PartialEq)]
pub enum Cloud {
    Aws,
    Gcp,
    Azure,
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
