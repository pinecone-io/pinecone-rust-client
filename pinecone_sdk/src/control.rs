use crate::pinecone::PineconeClient;
use crate::utils::errors::PineconeError;
use openapi::apis::manage_indexes_api;
use openapi::models::{
    CreateIndexRequest, CreateIndexRequestSpec, IndexList, IndexModel, ServerlessSpec,
};

pub use openapi::models::create_index_request::Metric;
pub use openapi::models::serverless_spec::Cloud;

impl PineconeClient {
    /// Creates serverless index
    pub async fn create_serverless_index(
        &self,
        name: &str,
        dimension: u32,
        metric: Metric,
        cloud: Cloud,
        region: &str,
    ) -> Result<IndexModel, PineconeError> {
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

        match manage_indexes_api::create_index(&self.openapi_config(), create_index_request).await {
            Ok(index) => Ok(index),
            Err(e) => Err(PineconeError::CreateIndexError { openapi_error: e }),
        }
    }
    /// Lists all indexes.
    ///
    /// The results include a description of all indexes in your project, including the
    /// index name, dimension, metric, status, and spec.
    ///
    /// :return: Returns an `IndexList` object, which is iterable and contains a
    ///     list of `IndexDescription` objects. It also has a convenience method `names()`
    ///     which returns a list of index names.
    ///
    /// ### Example
    ///
    /// ```
    /// # use pinecone_sdk::pinecone::PineconeClient;
    /// # use pinecone_sdk::utils::errors::PineconeError;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// # // Create a Pinecone client with the API key and controller host.
    /// # let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    /// #
    /// // List all indexes in the project.
    /// let index_list = pinecone.list_indexes();
    /// # Ok(())
    /// # }
    /// ```

    pub async fn list_indexes(&self) -> Result<IndexList, PineconeError> {
        match manage_indexes_api::list_indexes(&self.openapi_config()).await {
            Ok(response) => {
                println!("{:?}", response);
                Ok(response)
            }
            Err(e) => Err(PineconeError::ListIndexesError { openapi_error: e }),
        }
    }

    /// Deletes an index.
    ///
    /// ### Arguments
    /// * name: &str - The name of the index to be deleted.
    ///
    /// ### Return
    /// * Returns a `Result<(), PineconeError>` object. 
    ///
    /// ### Example
    /// ```
    /// # use pinecone_sdk::pinecone::PineconeClient;
    /// # use pinecone_sdk::control::{Cloud, Metric};
    /// # use pinecone_sdk::utils::errors::PineconeError;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// # let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    /// # let _ = pinecone.create_serverless_index("index-name", 2, Metric::Euclidean, Cloud::Aws, "us-west-2").await;
    /// let response = pinecone.delete_index("index-name").await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_index(&self, name: &str) -> Result<(), PineconeError> {
        match manage_indexes_api::delete_index(&self.openapi_config(), name).await {
            Ok(_) => Ok(()),
            Err(e) => Err(PineconeError::DeleteIndexError {
                name: name.to_string(),
                openapi_error: e,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    use openapi::models;
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

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(mockito::server_url()),
            None,
            None,
        );

        let create_index_request = pinecone
            .unwrap()
            .create_serverless_index("index_name", 10, Metric::Cosine, Cloud::Aws, "us-east-1")
            .await;
        assert!(create_index_request.is_ok());

        let create_index_req = create_index_request.unwrap();
        assert_eq!(create_index_req.name, "index_name");
        assert_eq!(create_index_req.dimension, 10);
        assert_eq!(
            create_index_req.metric,
            openapi::models::index_model::Metric::Euclidean
        );

        let spec = create_index_req.spec.serverless.unwrap();
        assert_eq!(spec.cloud, openapi::models::serverless_spec::Cloud::Aws);
        assert_eq!(spec.region, "us-east-1");
    }

    #[tokio::test]
    async fn test_create_serverless_index_defaults() {
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
                            "cloud": "gcp",
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

        let create_index_request = pinecone
            .unwrap()
            .create_serverless_index(
                "index_name",
                10,
                Default::default(),
                Default::default(),
                "us-east-1",
            )
            .await;
        assert!(create_index_request.is_ok());

        let create_index_req = create_index_request.unwrap();
        assert_eq!(create_index_req.name, "index_name");
        assert_eq!(create_index_req.dimension, 10);
        assert_eq!(
            create_index_req.metric,
            openapi::models::index_model::Metric::Cosine
        );

        let spec = create_index_req.spec.serverless.unwrap();
        assert_eq!(spec.cloud, openapi::models::serverless_spec::Cloud::Gcp);
        assert_eq!(spec.region, "us-east-1");
    }

    #[tokio::test]
    async fn test_list_indexes() -> Result<(), PineconeError> {
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
        let pinecone = PineconeClient::new(Some(api_key), Some(mockito::server_url()), None, None)
            .expect("Failed to create Pinecone instance");

        // Call list_indexes and verify the result
        let index_list = pinecone
            .list_indexes()
            .await
            .expect("Failed to list indexes");

        let expected = IndexList {
            // name: String, dimension: i32, metric: Metric, host: String, spec: models::IndexModelSpec, status: models::IndexModelStatus)
            indexes: Some(vec![
                IndexModel::new(
                    "index1".to_string(),
                    1536,
                    openapi::models::index_model::Metric::Cosine,
                    "host1".to_string(),
                    models::IndexModelSpec::default(),
                    models::IndexModelStatus::default(),
                ),
                IndexModel::new(
                    "index2".to_string(),
                    1536,
                    openapi::models::index_model::Metric::Cosine,
                    "host2".to_string(),
                    models::IndexModelSpec::default(),
                    models::IndexModelStatus::default(),
                ),
            ]),
        };
        assert_eq!(index_list, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_index() -> Result<(), PineconeError> {
        let _m = mock("DELETE", "/indexes/index_name")
            .with_status(204)
            .create();

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(mockito::server_url()),
            None,
            None,
        );

        let delete_index_request = pinecone.unwrap().delete_index("index_name").await;
        assert!(delete_index_request.is_ok());

        Ok(())
    }
}
