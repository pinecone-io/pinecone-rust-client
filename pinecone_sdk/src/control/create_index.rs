use core::panic;
use openapi::apis::manage_indexes_api::CreateIndexError;
use openapi::apis::Error;
use openapi::models::{CreateIndexRequest, CreateIndexRequestSpec, IndexModel, ServerlessSpec};
use openapi::models::create_index_request::Metric;
use openapi::models::serverless_spec::Cloud;
use crate::pinecone::Pinecone;
use crate::models::create_index_request_params::CreateServerlessIndexRequest;

impl Pinecone {
    pub async fn create_serverless_index(&self, params: CreateServerlessIndexRequest) -> Result<IndexModel, Error<CreateIndexError>> {
        let create_index_request = self.create_serverless_index_req(params);
        let response = openapi::apis::manage_indexes_api::create_index(
            &self.openapi_config(),
            create_index_request
        ).await?;
        Ok(response)
    }

    pub fn create_serverless_index_req(&self, params: CreateServerlessIndexRequest) -> CreateIndexRequest {
        // clean metric string
        let metric_enum = match &params.metric {
            Some(metric) => match metric.as_str() {
                "cosine" => Some(Metric::Cosine),
                "euclidean" => Some(Metric::Euclidean),
                "dotproduct" => Some(Metric::Dotproduct),
                _ => panic!("Invalid metric"), // TODO: handle error better
            },
            None => None,
        };

        // clean cloud string
        let cloud_enum = match &params.cloud {
            Some(cloud) => match cloud.as_str() {
                "gcp" => Cloud::Gcp,
                "aws" => Cloud::Aws,
                "azure" => Cloud::Azure,
                _ => panic!("Invalid cloud type"), // TODO: handle error better
            },
            None => Cloud::default(),
        };

        // create request specs
        let create_index_request_spec = CreateIndexRequestSpec {
            serverless: Some(Box::new(ServerlessSpec {
                cloud: cloud_enum,
                region: params.region,
            })),
            pod: None,
        };
        
        let create_index_request = CreateIndexRequest {
            name: params.name,
            dimension: params.dimension,
            metric: metric_enum,
            spec: Some(Box::new(create_index_request_spec)),
        };

        return create_index_request;
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
        let pinecone = Pinecone::new(Some("api_key".to_string()), Some("controller_url".to_string()), None, None);
        let params = CreateServerlessIndexRequest {
            name: "index_name".to_string(),
            dimension: 10,
            metric: Some("cosine".to_string()),
            cloud: Some("gcp".to_string()),
            region: "us-east-1".to_string(),
        };

        let create_index_request = pinecone.expect("REASON").create_serverless_index_req(params);
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

        let pinecone = Pinecone::new(Some("api_key".to_string()), Some(mockito::server_url()), None, None);
        let params = CreateServerlessIndexRequest {
            name: "index_name".to_string(),
            dimension: 10,
            metric: Some("euclidean".to_string()),
            cloud: Some("aws".to_string()),
            region: "us-east-1".to_string(),
        };

        let result = pinecone.expect("REASON").create_serverless_index(params).await;
        
        match result {
            Ok(index) => {
                assert_eq!(index.name, "index_name");
                assert_eq!(index.dimension, 10);
                assert_eq!(index.metric, openapi::models::index_model::Metric::Euclidean);
                let spec = *index.spec;
                let serverless_spec = spec.serverless.unwrap();
                assert_eq!(serverless_spec.cloud, openapi::models::serverless_spec::Cloud::Aws);
                assert_eq!(serverless_spec.region, "us-east-1");
            },
            Err(e) => panic!("{}", e),
        }
    }
}