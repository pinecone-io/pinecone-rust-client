use crate::models::create_index_request_params::CreateServerlessIndexRequest;
use crate::pinecone::Pinecone;
use crate::utils::errors::PineconeError;
use openapi::models::create_index_request::Metric;
use openapi::models::serverless_spec::Cloud;
use openapi::models::{CreateIndexRequest, CreateIndexRequestSpec, IndexModel, ServerlessSpec};

impl Pinecone {
    pub async fn create_serverless_index(
        &self,
        params: CreateServerlessIndexRequest,
    ) -> Result<IndexModel, PineconeError> {
        let create_index_request = self.create_serverless_index_req(params);
        match openapi::apis::manage_indexes_api::create_index(
            &self.openapi_config(),
            create_index_request?,
        )
        .await
        {
            Ok(index) => Ok(index),
            Err(e) => Err(PineconeError::CreateIndexError { openapi_error: e }),
        }
    }

    pub fn create_serverless_index_req(
        &self,
        params: CreateServerlessIndexRequest,
    ) -> Result<CreateIndexRequest, PineconeError> {
        let metric_enum = match &params.metric {
            Some(metric) => match metric.as_str() {
                "cosine" => Ok(Some(Metric::Cosine)),
                "euclidean" => Ok(Some(Metric::Euclidean)),
                "dotproduct" => Ok(Some(Metric::Dotproduct)),
                _ => Err(PineconeError::InvalidMetricError {
                    metric: metric.clone(),
                }),
            },
            None => Ok(Some(Metric::Cosine)),
        }?;

        // clean cloud string
        let cloud_enum = match &params.cloud {
            Some(cloud) => match cloud.as_str() {
                "gcp" => Ok(Cloud::Gcp),
                "aws" => Ok(Cloud::Aws),
                "azure" => Ok(Cloud::Azure),
                _ => Err(PineconeError::InvalidCloudError {
                    cloud: cloud.clone(),
                }),
            },
            None => Ok(Cloud::default()),
        }?;

        // create request specs
        let create_index_request_spec = CreateIndexRequestSpec {
            serverless: Some(Box::new(ServerlessSpec {
                cloud: cloud_enum,
                region: params.region,
            })),
            pod: None,
        };

        Ok(CreateIndexRequest {
            name: params.name,
            dimension: params.dimension,
            metric: metric_enum,
            spec: Some(Box::new(create_index_request_spec)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::panic;
    use mockito::mock;
    use openapi::models::serverless_spec;
    use serial_test::serial;
    use tokio;

    #[tokio::test]
    #[serial]
    async fn test_create_serverless_index_req() {
        let pinecone = Pinecone::new(
            Some("api_key".to_string()),
            Some("controller_url".to_string()),
            None,
            None,
        );
        let params = CreateServerlessIndexRequest {
            name: "index_name".to_string(),
            dimension: 10,
            metric: Some("cosine".to_string()),
            cloud: Some("gcp".to_string()),
            region: "us-east-1".to_string(),
        };

        let create_index_request = pinecone.unwrap().create_serverless_index_req(params);
        assert!(create_index_request.is_ok());

        let create_index_request = create_index_request.unwrap();
        assert_eq!(create_index_request.name, "index_name");
        assert_eq!(create_index_request.dimension, 10);
        assert_eq!(create_index_request.metric, Some(Metric::Cosine));

        let spec = create_index_request.spec.unwrap();
        let serverless_spec = spec.serverless.unwrap();
        assert_eq!(serverless_spec.cloud, serverless_spec::Cloud::Gcp);
        assert_eq!(serverless_spec.region, "us-east-1");
    }

    #[tokio::test]
    async fn test_create_serverless_index() {
        // Create a mock server
        let _m = mock("POST", "/indexes")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "name": "index_name",
                    "dimension": 10,
                    "metric": "euclidean",
                    "host": "host1",
                    "spec": {
                        "serverless": {
                            "cloud": "aws",
                            "region": "us-east-1"
                        }
                    },
                    "status": {
                        "ready": true,
                        "state": "Initializing"
                    }
                  }
            "#,
            )
            .create();

        let pinecone = Pinecone::new(
            Some("api_key".to_string()),
            Some(mockito::server_url()),
            None,
            None,
        );
        let params = CreateServerlessIndexRequest {
            name: "index_name".to_string(),
            dimension: 10,
            metric: Some("euclidean".to_string()),
            cloud: Some("aws".to_string()),
            region: "us-east-1".to_string(),
        };

        let result = pinecone.unwrap().create_serverless_index(params).await;

        match result {
            Ok(index) => {
                assert_eq!(index.name, "index_name");
                assert_eq!(index.dimension, 10);
                assert_eq!(
                    index.metric,
                    openapi::models::index_model::Metric::Euclidean
                );
                let spec = *index.spec;
                let serverless_spec = spec.serverless.unwrap();
                assert_eq!(
                    serverless_spec.cloud,
                    openapi::models::serverless_spec::Cloud::Aws
                );
                assert_eq!(serverless_spec.region, "us-east-1");
            }
            Err(e) => panic!("{}", e),
        }
    }
}
