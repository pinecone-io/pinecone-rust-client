use crate::models::create_index_request_params::{CreateIndexParams, Spec, Metric, Cloud};
use crate::pinecone::Pinecone;
use crate::utils::errors::PineconeError;
use openapi::models::create_index_request;
use openapi::models::serverless_spec;
use openapi::models::{CreateIndexRequest, CreateIndexRequestSpec, IndexModel, ServerlessSpec};

impl Pinecone {
    // Creates a new index based on the provided parameters
    pub async fn create_index(
        &self,
        params: CreateIndexParams
    ) -> Result<IndexModel, PineconeError> {
        
        // Check if creating serverless or pod-based index and call respective builder function
        match params.spec {
            Spec::Serverless { cloud, region } => {
                self.create_serverless_index(
                    params.name,
                    params.dimension,
                    params.metric,
                    cloud,
                    region
                ).await
            }
            Spec::Pod {
                environment: _,
                replicas: _,
                shards: _,
                pod_type: _,
                pods: _,
                metadata_config: _,
                source_collection: _,
            } => {
                // eventually change this to be pod index
                self.create_serverless_index(params.name, params.dimension, params.metric, Cloud::Aws, "".to_string()).await
            }
        }
    }

    // Creates serverless index
    async fn create_serverless_index(
        &self,
        name: String,
        dimension: u32, 
        metric: Metric, 
        cloud: Cloud, 
        region: String
    ) -> Result<IndexModel, PineconeError> {

        // clean metric enum
        let metric_enum = match metric {
            Metric::Cosine => Some(create_index_request::Metric::Cosine),
            Metric::Dotproduct => Some(create_index_request::Metric::Dotproduct),
            Metric::Euclidean => Some(create_index_request::Metric::Euclidean),
        };

        // clean cloud enum
        let cloud_enum = match cloud {
            Cloud::Aws => serverless_spec::Cloud::Aws,
            Cloud::Gcp => serverless_spec::Cloud::Gcp,
            Cloud::Azure => serverless_spec::Cloud::Azure,
        };

        // create request specs
        let create_index_request_spec = CreateIndexRequestSpec {
            serverless: Some(Box::new(ServerlessSpec {
                cloud: cloud_enum,
                region,
            })),
            pod: None,
        };

        let create_index_request = CreateIndexRequest {
            name,
            dimension: dimension.try_into().unwrap(),
            metric: metric_enum,
            spec: Some(Box::new(create_index_request_spec)),
        };

        match openapi::apis::manage_indexes_api::create_index(
            &self.openapi_config(),
            create_index_request,
        )
        .await
        {
            Ok(index) => Ok(index),
            Err(e) => Err(PineconeError::CreateIndexError { openapi_error: e }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::panic;
    use mockito::mock;
    use tokio;

    #[tokio::test]
    async fn test_create_serverless_index() {
        let _m = mock("POST", "/indexes")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "name": "index_name",
                    "dimension": 10,
                    "metric": "cosine",
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
        let name = "index_name".to_string();
        let dimension = 10;
        
        let create_index_request = pinecone.unwrap().create_serverless_index(
            name,
            dimension,
            Metric::Cosine,
            Cloud::Aws,
            "us-east-1".to_string()
        ).await;
        assert!(create_index_request.is_ok());

        let create_index_req = create_index_request.unwrap();
        assert_eq!(create_index_req.name, "index_name");
        assert_eq!(create_index_req.dimension, 10);
        assert_eq!(create_index_req.metric, openapi::models::index_model::Metric::Cosine);

        let spec = create_index_req.spec.serverless.unwrap();
        assert_eq!(spec.cloud, openapi::models::serverless_spec::Cloud::Aws);
        assert_eq!(spec.region, "us-east-1");
    }

    #[tokio::test]
    async fn test_create_index_serverless() {
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
        let params = CreateIndexParams {
            name: "index_name".to_string(),
            dimension: 10,
            metric: Metric::Euclidean,
            spec: Spec::Serverless {
                cloud: Cloud::Aws,
                region: "us-east-1".to_string(),
            },
        };

        let result = pinecone.unwrap().create_index(params).await;

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
