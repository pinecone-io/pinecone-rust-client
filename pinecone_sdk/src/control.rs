use crate::pinecone::PineconeClient;
use crate::utils::errors::PineconeError;
use openapi::apis::manage_indexes_api;
use openapi::models::{
    CollectionModel, CreateCollectionRequest, CreateIndexRequest, CreateIndexRequestSpec,
    IndexList, IndexModel, PodSpec, PodSpecMetadataConfig, ServerlessSpec,
};
use std::time::Duration;

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
    /// let create_index_response = pinecone.create_serverless_index(
    ///     "create-index", // Name of the index
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

    /// Creates a Pinecone pod index.
    ///
    /// ### Arguments
    /// * `name: String` - The name of the index
    /// * `dimension: u32` - The dimension of the index
    /// * `metric: Metric` - The metric to use for the index
    /// * `environment: String` - The environment to use for the index
    /// * `replicas: Option<i32>` - The number of replicas to use for the index
    /// * `shards: Option<i32>` - The number of shards to use for the index
    /// * `pod_type: String` - The type of pod to use for the index
    /// * `pods: i32` - The number of pods to use for the index
    /// * `indexed: Option<Vec<String>>` - The metadata fields to index
    /// * `source_collection: Option<String>` - The source collection to use for the index
    /// * `timeout: Option<u32>` - The timeout for the request
    ///
    /// ### Return
    /// * Returns a `Result<IndexModel, PineconeError>` object.
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::utils::errors::PineconeError;
    /// use pinecone_sdk::control::{Metric, Cloud};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError> {
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// // Create a pod index.
    /// let create_index_response = pinecone.create_pod_index(
    ///     "index_name", // Name of the index
    ///     10, // Dimension of the index
    ///     Metric::Cosine, // Distance metric
    ///     "us-east-1-aws", // Environment
    ///     Some(1), // Number of replicas
    ///     Some(1), // Number of shards
    ///     "p1.x1", // Pod type
    ///     1, // Number of pods
    ///     Some( // Metadata fields to index
    ///         vec!["genre".to_string(),
    ///         "title".to_string(),
    ///         "imdb_rating".to_string()]),
    ///     Some("example-collection".to_string()), // Source collection
    ///     None // Request timeout
    /// )
    /// .await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Include any additional technical notes here.
    pub async fn create_pod_index(
        &self,
        name: &str,
        dimension: u32,
        metric: Metric,
        environment: &str,
        replicas: Option<i32>,
        shards: Option<i32>,
        pod_type: &str,
        pods: i32,
        indexed: Option<Vec<String>>,
        source_collection: Option<String>,
        timeout: Option<u32>,
    ) -> Result<IndexModel, PineconeError> {
        let pod_spec = PodSpec {
            environment: environment.to_string(),
            replicas,
            shards,
            pod_type: pod_type.to_string(),
            pods,
            metadata_config: Some(Box::new(PodSpecMetadataConfig { indexed })),
            source_collection,
        };

        let spec = CreateIndexRequestSpec {
            serverless: None,
            pod: Some(Box::new(pod_spec)),
        };

        let create_index_request = CreateIndexRequest {
            name: name.to_string(),
            dimension: dimension.try_into().unwrap(),
            metric: Some(metric),
            spec: Some(Box::new(spec)),
        };

        match timeout {
            Some(timeout) => {
                let timeout = std::time::Duration::from_secs(timeout.into());
                match tokio::time::timeout(
                    timeout,
                    self.create_pod_index_call(create_index_request),
                )
                .await
                {
                    Ok(index) => Ok(index?),
                    Err(_) => Err(PineconeError::TimeoutError),
                }
            }
            None => self.create_pod_index_call(create_index_request).await,
        }
    }

    // Helper function to make async create_index call
    async fn create_pod_index_call(
        &self,
        create_index_request: CreateIndexRequest,
    ) -> Result<IndexModel, PineconeError> {
        match manage_indexes_api::create_index(&self.openapi_config(), create_index_request).await {
            Ok(index) => Ok(index),
            Err(e) => Err(PineconeError::CreateIndexError { openapi_error: e }),
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

    // Test function to mock a timeout error
    async fn mock_timeout(&self, timeout: u32) -> Result<(), PineconeError> {
        let timeout = std::time::Duration::from_secs(timeout.into());
        match tokio::time::timeout(timeout, tokio::time::sleep(Duration::from_secs(1000000000)))
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(PineconeError::TimeoutError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
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
        )
        .unwrap();

        let create_index_response = pinecone
            .create_serverless_index("index_name", 10, Metric::Cosine, Cloud::Aws, "us-east-1")
            .await
            .expect("Failed to create serverless index");

        assert_eq!(create_index_response.name, "index_name");
        assert_eq!(create_index_response.dimension, 10);
        assert_eq!(
            create_index_response.metric,
            openapi::models::index_model::Metric::Euclidean
        );

        let spec = create_index_response.spec.serverless.unwrap();
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
        )
        .unwrap();

        let create_index_response = pinecone
            .create_serverless_index(
                "index_name",
                10,
                Default::default(),
                Default::default(),
                "us-east-1",
            )
            .await
            .expect("Failed to create serverless index");

        assert_eq!(create_index_response.name, "index_name");
        assert_eq!(create_index_response.dimension, 10);
        assert_eq!(
            create_index_response.metric,
            openapi::models::index_model::Metric::Cosine
        );

        let spec = create_index_response.spec.serverless.unwrap();
        assert_eq!(spec.cloud, openapi::models::serverless_spec::Cloud::Gcp);
        assert_eq!(spec.region, "us-east-1");

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
    async fn test_create_pod_index() -> Result<(), PineconeError> {
        let _m = mock("POST", "/indexes")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "name": "test-index",
                    "dimension": 1536,
                    "metric": "euclidean",
                    "host": "semantic-search-c01b5b5.svc.us-west1-gcp.pinecone.io",
                    "spec": {
                        "pod": {
                        "environment": "us-east-1-aws",
                        "metadata_config": {
                            "indexed": [
                                "genre",
                                "title",
                                "imdb_rating"
                            ]
                        },
                        "pod_type": "p1.x1",
                        "pods": 1,
                        "replicas": 1,
                        "shards": 1
                        }
                    },
                    "status": {
                        "ready": true,
                        "state": "ScalingUpPodSize"
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
        )
        .unwrap();

        let create_index_response = pinecone
            .create_pod_index(
                "test-index",
                1536,
                Metric::Euclidean,
                "us-east-1-aws",
                Some(1),
                Some(1),
                "p1.x1",
                1,
                Some(vec![
                    "genre".to_string(),
                    "title".to_string(),
                    "imdb_rating".to_string(),
                ]),
                Some("example-collection".to_string()),
                None,
            )
            .await
            .expect("Failed to create pod index");

        assert_eq!(create_index_response.name, "test-index");
        assert_eq!(create_index_response.dimension, 1536);
        assert_eq!(
            create_index_response.metric,
            openapi::models::index_model::Metric::Euclidean
        );

        let pod_spec = create_index_response.spec.pod.as_ref().unwrap();
        assert_eq!(pod_spec.environment, "us-east-1-aws");
        assert_eq!(pod_spec.pod_type, "p1.x1");
        assert_eq!(
            pod_spec.metadata_config.as_ref().unwrap().indexed,
            Some(vec![
                "genre".to_string(),
                "title".to_string(),
                "imdb_rating".to_string()
            ])
        );
        assert_eq!(pod_spec.pods, 1);
        assert_eq!(pod_spec.replicas, Some(1));
        assert_eq!(pod_spec.shards, Some(1));

        Ok(())
    }

    #[tokio::test]
    async fn test_create_pod_index_with_defaults() -> Result<(), PineconeError> {
        let _m = mock("POST", "/indexes")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "name": "test-index",
                    "dimension": 1536,
                    "metric": "cosine",
                    "host": "semantic-search-c01b5b5.svc.us-west1-gcp.pinecone.io",
                    "spec": {
                        "pod": {
                        "environment": "us-east-1-aws",
                        "metadata_config": {},
                        "pod_type": "p1.x1",
                        "pods": 1,
                        "replicas": 1,
                        "shards": 1
                        }
                    },
                    "status": {
                        "ready": true,
                        "state": "ScalingUpPodSize"
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
        )
        .unwrap();

        let create_index_response = pinecone
            .create_pod_index(
                "test-index",
                1536,
                Default::default(),
                "us-east-1-aws",
                None,
                None,
                "p1.x1",
                1,
                None,
                None,
                None,
            )
            .await
            .expect("Failed to create pod index");

        assert_eq!(create_index_response.name, "test-index");
        assert_eq!(create_index_response.dimension, 1536);
        assert_eq!(
            create_index_response.metric,
            openapi::models::index_model::Metric::Cosine
        );

        let pod_spec = create_index_response.spec.pod.as_ref().unwrap();
        assert_eq!(pod_spec.environment, "us-east-1-aws");
        assert_eq!(pod_spec.pod_type, "p1.x1");
        assert_eq!(pod_spec.metadata_config.as_ref().unwrap().indexed, None);
        assert_eq!(pod_spec.pods, 1);
        assert_eq!(pod_spec.replicas, Some(1));
        assert_eq!(pod_spec.shards, Some(1));

        Ok(())
    }

    #[tokio::test]
    async fn test_create_pod_index_timeout() -> Result<(), PineconeError> {
        let _m = mock("POST", "/indexes")
            .with_status(201)
            .with_header("content-type", "application/json")
            .create();

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(mockito::server_url()),
            None,
            None,
        )
        .unwrap();

        let create_index_response = pinecone.mock_timeout(1).await;
        assert_eq!(
            create_index_response.unwrap_err(),
            PineconeError::TimeoutError
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
}
