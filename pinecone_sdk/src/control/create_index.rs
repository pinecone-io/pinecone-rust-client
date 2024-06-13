use crate::pinecone::PineconeClient;
use crate::utils::errors::PineconeError;
use openapi::models::serverless_spec::Cloud;
use openapi::models::create_index_request::Metric;
use openapi::models::{CreateIndexRequest, CreateIndexRequestSpec, IndexModel, ServerlessSpec};

impl PineconeClient {
    /// Creates serverless index
    pub async fn create_serverless_index(
        &self,
        name: &str,
        dimension: u32, 
        metric: Option<Metric>, 
        cloud: Option<Cloud>,
        region: &str
    ) -> Result<IndexModel, PineconeError> {
        // use defaults
        let metric = metric.unwrap_or(Default::default());
        let cloud = cloud.unwrap_or(Default::default());

        // create request specs
        let create_index_request_spec = CreateIndexRequestSpec {
            serverless: Some(Box::new(ServerlessSpec {
                cloud,
                region: region.to_string(),
            })),
            pod: None,
        };

        let create_index_request = CreateIndexRequest {
            name: name.to_string(),
            dimension: dimension.try_into().unwrap(),
            metric: Some(metric),
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

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(mockito::server_url()),
            None,
            None,
        );
        
        let create_index_request = pinecone.unwrap().create_serverless_index(
            "index_name",
            10,
            Some(Metric::Cosine),
            Some(Cloud::Aws),
            "us-east-1"
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
}
