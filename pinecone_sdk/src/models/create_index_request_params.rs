use crate::utils::errors::PineconeError;

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

    // constructs builder for CreateIndexParams
    pub fn builder() -> CreateIndexParamsBuilder {
        CreateIndexParamsBuilder::new()
    }
}

pub struct CreateIndexParamsBuilder {
    name: Option<String>,
    dimension: Option<u32>,
    metric: Option<Metric>,
    spec: Option<Spec>,
}

impl CreateIndexParamsBuilder {
    pub fn new() -> CreateIndexParamsBuilder {
        CreateIndexParamsBuilder {
            name: None,
            dimension: None,
            metric: None,
            spec: None,
        }
    }

    pub fn with_name(mut self, name: &str) -> CreateIndexParamsBuilder {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_dimension(mut self, dimension: u32) -> CreateIndexParamsBuilder {
        // want to eventually throw an error if dimension is 0?
        self.dimension = Some(dimension);
        self
    }

    pub fn with_metric(mut self, metric: Metric) -> CreateIndexParamsBuilder {
        self.metric = Some(metric);
        self
    }

    pub fn with_spec(mut self, spec: Spec) -> CreateIndexParamsBuilder {
        self.spec = Some(spec);
        self
    }

    // constructs CreateIndexParams from CreateIndexParamsBuilder fields
    pub fn build(self) -> Result<CreateIndexParams, PineconeError> {
        // required parameters
        let name = self.name.ok_or(PineconeError::MissingNameError)?;
        let dimension = self.dimension.ok_or(PineconeError::MissingDimensionError)?;
        let spec = self.spec.ok_or(PineconeError::MissingSpecError)?;

        Ok(CreateIndexParams {
            name,
            dimension,
            metric: self.metric.unwrap_or(Metric::Cosine),
            spec,
        })
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

    #[tokio::test]
    async fn test_create_index_params_builder() {
        let create_index_params = CreateIndexParams::builder()
            .with_name("test_index")
            .with_dimension(10)
            .with_metric(Metric::Cosine)
            .with_spec(Spec::Serverless {
                cloud: Cloud::Aws,
                region: "us-west-2".to_string(),
            })
            .build()
            .unwrap();

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

    #[tokio::test]
    async fn test_builder_missing_metric() {
        let create_index_params = CreateIndexParams::builder()
            .with_name("test_index")
            .with_dimension(10)
            .with_spec(Spec::Serverless {
                cloud: Cloud::Aws,
                region: "us-west-2".to_string(),
            })
            .build()
            .unwrap();

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

    #[tokio::test]
    async fn test_missing_name() {
        let create_index_params = CreateIndexParams::builder()
            .with_dimension(10)
            .with_metric(Metric::Cosine)
            .build();

        assert!(create_index_params.is_err());
        assert_eq!(create_index_params, Err(PineconeError::MissingNameError));
    }

    #[tokio::test]
    async fn test_missing_dimension() {
        let create_index_params = CreateIndexParams::builder()
            .with_name("test_index")
            .with_metric(Metric::Cosine)
            .build();

        assert!(create_index_params.is_err());
        assert_eq!(
            create_index_params,
            Err(PineconeError::MissingDimensionError)
        );
    }

    #[tokio::test]
    async fn test_missing_spec() {
        let create_index_params = CreateIndexParams::builder()
            .with_name("test_index")
            .with_dimension(10)
            .build();

        assert!(create_index_params.is_err());
        assert_eq!(create_index_params, Err(PineconeError::MissingSpecError));
    }
}
