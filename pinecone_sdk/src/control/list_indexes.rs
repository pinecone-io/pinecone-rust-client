use crate::pinecone::Pinecone;
use openapi::apis::manage_indexes_api;
use openapi::apis::manage_indexes_api::ListIndexesError;
use openapi::apis::Error;
use openapi::models;

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
    use crate::config::Config;
    use crate::control::list_indexes::models::index_model::Metric;
    use mockito::mock;
    use openapi::apis::configuration::ApiKey;
    use openapi::apis::configuration::Configuration;
    use openapi::models::IndexList;
    use openapi::models::IndexModel;
    use tokio;

    #[tokio::test]
    async fn test_list_indexes() {
        // Create a mock server
        let _m = mock("GET", "/indexes")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
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
            "#,
            )
            .create();

        // Construct Pinecone instance with the mock server URL
        let api_key = "test_api_key".to_string();
        let pinecone = Pinecone::new(Some(api_key), Some(mockito::server_url()), None, None)
            .expect("Failed to create Pinecone instance");

        // Call list_indexes and verify the result
        let result = pinecone.list_indexes().await;

        match result {
            Ok(index_list) => {
                let expected = IndexList {
                    // name: String, dimension: i32, metric: Metric, host: String, spec: models::IndexModelSpec, status: models::IndexModelStatus)
                    indexes: Some(vec![
                        IndexModel::new(
                            "index1".to_string(),
                            1536,
                            Metric::Cosine,
                            "host1".to_string(),
                            models::IndexModelSpec::default(),
                            models::IndexModelStatus::default(),
                        ),
                        IndexModel::new(
                            "index2".to_string(),
                            1536,
                            Metric::Cosine,
                            "host2".to_string(),
                            models::IndexModelSpec::default(),
                            models::IndexModelStatus::default(),
                        ),
                    ]),
                };
                assert_eq!(index_list, expected);
            }
            Err(err) => panic!("Expected Ok, got Err: {:?}", err),
        }
    }
}
