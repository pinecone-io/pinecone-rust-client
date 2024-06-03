use crate::pinecone::Pinecone;
use openapi::apis::manage_indexes_api;
use openapi::models;
use openapi::apis::Error;
use openapi::apis::manage_indexes_api::ListIndexesError;

impl Pinecone {
    pub async fn list_indexes(&self) -> Result<models::IndexList, Error<ListIndexesError>> {
        let response = manage_indexes_api::list_indexes(self.openapi_config()).await?;
        println!("{:?}", response);
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use mockito::mock;
    use openapi::models::IndexList;

    #[tokio::test]
    async fn test_list_indexes() {
        // Create a mock server
        let _m = mock("GET", "/indexes")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"
                {
                    "indexes": [
                        {
                            "name": "index1",
                            "dimension": 1536,
                            "metric": "cosine",
                            "host": "host1",
                            "spec": {},
                            "status": {
                                "ready": false,
                                "state": "Initializing"
                            }
                        },
                        {
                            "name": "index2",
                            "dimension": 1536,
                            "metric": "cosine",
                            "host": "host2",
                            "spec": {},
                            "status": {
                                "ready": false,
                                "state": "Initializing"
                            }
                        }
                    ]
                }
            "#)
            .create();

        // Construct Pinecone instance with the mock server URL
        let api_key = "test_api_key".to_string();
        let config = Config::new(api_key.clone());
        let openapi_config = Configuration {
            base_path: mockito::server_url(), // Use mock server URL
            user_agent: Some("pinecone-rust-client".to_string()),
            api_key: Some(ApiKey {
                prefix: None,
                key: api_key,
            }),
            ..Default::default()
        };
        let pinecone = Pinecone { config, openapi_config };

        // Call list_indexes and verify the result
        let result = pinecone.list_indexes().await;

        match result {
            Ok(index_list) => {
                let expected = IndexList {
                    // name: String, dimension: i32, metric: Metric, host: String, spec: models::IndexModelSpec, status: models::IndexModelStatus)
                    indexes: Some(vec![
                                IndexModel::new("index1".to_string(), 1536, Metric::Cosine, "host1".to_string(), models::IndexModelSpec::default(), models::IndexModelStatus::default()),
                                IndexModel::new("index2".to_string(), 1536, Metric::Cosine, "host2".to_string(), models::IndexModelSpec::default(), models::IndexModelStatus::default()),
                            ]),
                };
                assert_eq!(index_list, expected);
            }
            Err(err) => panic!("Expected Ok, got Err: {:?}", err),
        }
    }
}
