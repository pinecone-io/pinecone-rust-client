use crate::pinecone::PineconeClient;
use crate::utils::errors::PineconeError;
use openapi::apis::manage_indexes_api;
use openapi::models::{
    CollectionModel, CreateCollectionRequest, CreateIndexRequest, CreateIndexRequestSpec,
    IndexList, IndexModel, ServerlessSpec,
};

pub use openapi::models::create_index_request::Metric;
pub use openapi::models::serverless_spec::Cloud;

impl PineconeClient {
    /// Creates a serverless index.
    ///
    /// ### Arguments
    /// * `name: &str` - Name of the index to create.
    /// * `dimension: u32` - Dimension of the vectors to be inserted in the index.
    /// * `metric: Metric` - The distance metric to be used for similarity search.
    /// * `cloud: Cloud` - The public cloud where you would like your index hosted.
    /// * `region: &str` - The region where you would like your index to be created.
    ///
    /// ### Return
    /// * `Result<IndexModel, PineconeError>`
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::utils::errors::PineconeError;
    /// use pinecone_sdk::control::{Metric, Cloud};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// // Create an index.
    /// let create_index_request = pinecone.create_serverless_index(
    ///     "index-name", // Name of the index
    ///     10, // Dimension of the vectors
    ///     Metric::Cosine, // Distance metric
    ///     Cloud::Aws, // Cloud provider
    ///     "us-east-1" // Region
    /// ).await.unwrap();
    ///
    /// # Ok(())
    /// # }
    /// ```
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

    /// Describes an index.
    ///
    /// ### Arguments
    /// * `name: &str` - Name of the index to describe.
    ///
    /// ### Return
    /// * `Result<IndexModel, PineconeError>`
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// // Describe an index in the project.
    /// let index = pinecone.describe_index("index-name").await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn describe_index(&self, name: &str) -> Result<IndexModel, PineconeError> {
        match manage_indexes_api::describe_index(&self.openapi_config(), name).await {
            Ok(index) => Ok(index),
            Err(e) => Err(PineconeError::DescribeIndexError {
                name: name.to_string(),
                openapi_error: e,
            }),
        }
    }

    /// Lists all indexes.
    ///
    /// The results include a description of all indexes in your project, including the
    /// index name, dimension, metric, status, and spec.
    ///
    /// ### Return
    /// * `Result<IndexList, PineconeError>`
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// // List all indexes in the project.
    /// let index_list = pinecone.list_indexes().await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_indexes(&self) -> Result<IndexList, PineconeError> {
        match manage_indexes_api::list_indexes(&self.openapi_config()).await {
            Ok(index_list) => Ok(index_list),
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
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::control::{Cloud, Metric};
    /// use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
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

    /// Creates a collection from an index.
    ///
    /// ### Arguments
    /// * `name: &str` - Name of the collection to create.
    /// * `source: &str` - Name of the index to be used as the source for the collection.
    ///
    /// ### Return
    /// * `Result<CollectionModel, PineconeError>`
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// // Describe an index in the project.
    /// let collection = pinecone.create_collection("collection-name", "index-name").await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_collection(
        &self,
        name: &str,
        source: &str,
    ) -> Result<CollectionModel, PineconeError> {
        let create_collection_request = CreateCollectionRequest {
            name: name.to_string(),
            source: source.to_string(),
        };
        match manage_indexes_api::create_collection(
            &self.openapi_config(),
            create_collection_request,
        )
        .await
        {
            Ok(collection) => Ok(collection),
            Err(e) => Err(PineconeError::CreateCollectionError {
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
    use openapi::apis::{Error as OpenAPIError, ResponseContent};
    use openapi::models::{self, collection_model::Status, IndexList};
    use tokio;

    #[tokio::test]
    async fn test_create_serverless_index() -> Result<(), PineconeError> {
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
            .await
            .expect("Failed to create index");

        assert_eq!(create_index_request.name, "index_name");
        assert_eq!(create_index_request.dimension, 10);
        assert_eq!(
            create_index_request.metric,
            openapi::models::index_model::Metric::Euclidean
        );

        let spec = create_index_request.spec.serverless.unwrap();
        assert_eq!(spec.cloud, openapi::models::serverless_spec::Cloud::Aws);
        assert_eq!(spec.region, "us-east-1");

        Ok(())
    }

    #[tokio::test]
    async fn test_create_serverless_index_defaults() -> Result<(), PineconeError> {
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
            .await
            .expect("Failed to create index");

        assert_eq!(create_index_request.name, "index_name");
        assert_eq!(create_index_request.dimension, 10);
        assert_eq!(
            create_index_request.metric,
            openapi::models::index_model::Metric::Cosine
        );

        let spec = create_index_request.spec.serverless.unwrap();
        assert_eq!(spec.cloud, openapi::models::serverless_spec::Cloud::Gcp);
        assert_eq!(spec.region, "us-east-1");

        Ok(())
    }

    #[tokio::test]
    async fn test_create_serverless_index_server_error() -> Result<(), PineconeError> {
        let _m = mock("POST", "/indexes").with_status(500).create();

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(mockito::server_url()),
            None,
            None,
        )
        .unwrap();

        let create_index_response = pinecone
            .create_serverless_index("index_name", 10, Metric::Cosine, Cloud::Aws, "us-east-1")
            .await
            .expect_err("Expected create_index to return an error");

        assert_eq!(
            create_index_response,
            PineconeError::CreateIndexError {
                openapi_error: OpenAPIError::ResponseError(ResponseContent {
                    status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                    content: "Internal Server Error".to_string(),
                    entity: None,
                })
            }
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_describe_index() -> Result<(), PineconeError> {
        // Create a mock server
        let _m = mock("GET", "/indexes/serverless-index")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "name": "serverless-index",
                    "metric": "cosine",
                    "dimension": 1536,
                    "status": {
                       "ready": true,
                       "state": "Ready"
                    },
                    "host": "serverless-index-4zo0ijk.svc.us-east1-aws.pinecone.io",
                    "spec": {
                       "serverless": {
                          "region": "us-east-1",
                          "cloud": "aws"
                       }
                    }
                 }
            "#,
            )
            .create();

        // Construct Pinecone instance with the mock server URL
        let api_key = "test_api_key".to_string();
        let pinecone = PineconeClient::new(Some(api_key), Some(mockito::server_url()), None, None)
            .expect("Failed to create Pinecone instance");

        // Call describe_index and verify the result
        let index = pinecone
            .describe_index("serverless-index")
            .await
            .expect("Failed to describe index");

        let expected = IndexModel {
            name: "serverless-index".to_string(),
            metric: openapi::models::index_model::Metric::Cosine,
            dimension: 1536,
            status: Box::new(openapi::models::IndexModelStatus {
                ready: true,
                state: openapi::models::index_model_status::State::Ready,
            }),
            host: "serverless-index-4zo0ijk.svc.us-east1-aws.pinecone.io".to_string(),
            spec: Box::new(models::IndexModelSpec {
                serverless: Some(Box::new(models::ServerlessSpec {
                    cloud: openapi::models::serverless_spec::Cloud::Aws,
                    region: "us-east-1".to_string(),
                })),
                pod: None,
            }),
        };
        assert_eq!(index, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_describe_index_invalid_name() -> Result<(), PineconeError> {
        let _m = mock("GET", "/indexes/invalid-index")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "error": "Index not found"
                }
            "#,
            )
            .create();

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(mockito::server_url()),
            None,
            None,
        )
        .unwrap();

        let describe_index_response = pinecone
            .describe_index("invalid-index")
            .await
            .expect_err("Expected describe_index to return an error");

        assert_eq!(
            describe_index_response,
            PineconeError::DescribeIndexError {
                name: "invalid-index".to_string(),
                openapi_error: OpenAPIError::ResponseError(ResponseContent {
                    status: reqwest::StatusCode::NOT_FOUND,
                    content: "Index not found".to_string(),
                    entity: None,
                })
            }
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_describe_index_server_error() -> Result<(), PineconeError> {
        let _m = mock("GET", "/indexes/serverless-index")
            .with_status(500)
            .create();

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(mockito::server_url()),
            None,
            None,
        )
        .unwrap();

        let describe_index_response = pinecone
            .describe_index("serverless-index")
            .await
            .expect_err("Expected describe_index to return an error");

        assert_eq!(
            describe_index_response,
            PineconeError::DescribeIndexError {
                name: "serverless-index".to_string(),
                openapi_error: OpenAPIError::ResponseError(ResponseContent {
                    status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                    content: "Internal Server Error".to_string(),
                    entity: None,
                })
            }
        );

        Ok(())
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
    async fn test_list_indexes_server_error() -> Result<(), PineconeError> {
        let _m = mock("GET", "/indexes").with_status(500).create();

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(mockito::server_url()),
            None,
            None,
        )
        .unwrap();

        let list_indexes_response = pinecone
            .list_indexes()
            .await
            .expect_err("Expected list_indexes to return an error");

        assert_eq!(
            list_indexes_response,
            PineconeError::ListIndexesError {
                openapi_error: OpenAPIError::ResponseError(ResponseContent {
                    status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                    content: "Internal Server Error".to_string(),
                    entity: None,
                })
            }
        );

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

    #[tokio::test]
    async fn test_delete_index_invalid_name() -> Result<(), PineconeError> {
        let _m = mock("DELETE", "/indexes/invalid-index")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "error": "Index not found"
                }
            "#,
            )
            .create();

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(mockito::server_url()),
            None,
            None,
        )
        .unwrap();

        let delete_index_response = pinecone
            .delete_index("invalid-index")
            .await
            .expect_err("Expected delete_index to return an error");

        assert_eq!(
            delete_index_response,
            PineconeError::DeleteIndexError {
                name: "invalid-index".to_string(),
                openapi_error: OpenAPIError::ResponseError(ResponseContent {
                    status: reqwest::StatusCode::NOT_FOUND,
                    content: "Index not found".to_string(),
                    entity: None,
                })
            }
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_index_server_error() -> Result<(), PineconeError> {
        let _m = mock("DELETE", "/indexes/index_name")
            .with_status(500)
            .create();

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(mockito::server_url()),
            None,
            None,
        )
        .unwrap();

        let delete_index_response = pinecone
            .delete_index("invalid-index")
            .await
            .expect_err("Expected delete_index to return an error");

        assert_eq!(
            delete_index_response,
            PineconeError::DeleteIndexError {
                name: "invalid-index".to_string(),
                openapi_error: OpenAPIError::ResponseError(ResponseContent {
                    status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                    content: "Internal Server Error".to_string(),
                    entity: None,
                })
            }
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_create_collection() -> Result<(), PineconeError> {
        // Create a mock server
        let _m = mock("POST", "/collections")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "name": "example-collection",
                    "size": 10000000,
                    "status": "Initializing",
                    "dimension": 1536,
                    "vector_count": 120000,
                    "environment": "us-east1-gcp"
                  }
            "#,
            )
            .create();

        // Construct Pinecone instance with the mock server URL
        let api_key = "test_api_key".to_string();
        let pinecone = PineconeClient::new(Some(api_key), Some(mockito::server_url()), None, None)
            .expect("Failed to create Pinecone instance");

        // Call create_collection and verify the result
        let collection = pinecone
            .create_collection("collection1", "index1")
            .await
            .expect("Failed to create collection");

        let expected = CollectionModel {
            name: "example-collection".to_string(),
            size: Some(10000000),
            status: Status::Initializing,
            dimension: Some(1536),
            vector_count: Some(120000),
            environment: "us-east1-gcp".to_string(),
        };
        assert_eq!(collection, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_create_collection_invalid_name() -> Result<(), PineconeError> {
        let _m = mock("POST", "/collections")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "error": "Index not found"
                }
            "#,
            )
            .create();

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(mockito::server_url()),
            None,
            None,
        )
        .unwrap();

        let create_collection_response = pinecone
            .create_collection("invalid_collection", "valid-index")
            .await
            .expect_err("Expected create_collection to return an error");

        assert_eq!(
            create_collection_response,
            PineconeError::CreateCollectionError {
                name: "invalid_collection".to_string(),
                openapi_error: OpenAPIError::ResponseError(ResponseContent {
                    status: reqwest::StatusCode::BAD_REQUEST,
                    content: "Bad Request".to_string(),
                    entity: None,
                })
            }
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_create_collection_server_error() -> Result<(), PineconeError> {
        let _m = mock("POST", "/collections")
            .with_status(500)
            .create();

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(mockito::server_url()),
            None,
            None,
        )
        .unwrap();

        let create_collection_response = pinecone
            .create_collection("collection-name", "index1")
            .await
            .expect_err("Expected create_collection to return an error");

        assert_eq!(
            create_collection_response,
            PineconeError::CreateCollectionError {
                name: "collection-name".to_string(),
                openapi_error: OpenAPIError::ResponseError(ResponseContent {
                    status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                    content: "Internal Server Error".to_string(),
                    entity: None,
                })
            }
        );

        Ok(())
    }
}
