use crate::pinecone::PineconeClient;
use crate::utils::errors::PineconeError;
use openapi::apis::manage_indexes_api;

pub use openapi::models::create_index_request::Metric;
pub use openapi::models::serverless_spec::Cloud;
pub use openapi::models::{
    CollectionList, CollectionModel, CreateCollectionRequest, CreateIndexRequest,
    CreateIndexRequestSpec, IndexList, IndexModel, PodSpec, PodSpecMetadataConfig, ServerlessSpec,
};

impl PineconeClient {
    /// Creates a serverless index.
    ///
    /// ### Arguments
    /// * `name: &str` - Name of the index to create.
    /// * `dimension: i32` - Dimension of the vectors to be inserted in the index.
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
    /// use pinecone_sdk::control::{Metric, Cloud, IndexModel};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// // Create an index.
    /// let create_index_response: Result<IndexModel, PineconeError> = pinecone.create_serverless_index(
    ///     "index-name", // Name of the index
    ///     10, // Dimension of the vectors
    ///     Metric::Cosine, // Distance metric
    ///     Cloud::Aws, // Cloud provider
    ///     "us-east-1" // Region
    /// ).await;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_serverless_index(
        &self,
        name: &str,
        dimension: i32,
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
            dimension,
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
    /// use pinecone_sdk::control::IndexModel;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// // Describe an index in the project.
    /// let describe_index_response: Result<IndexModel, PineconeError> = pinecone.describe_index("index-name").await;
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
    /// use pinecone_sdk::control::IndexList;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// // List all indexes in the project.
    /// let index_list_response: Result<IndexList, PineconeError> = pinecone.list_indexes().await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_indexes(&self) -> Result<IndexList, PineconeError> {
        match manage_indexes_api::list_indexes(&self.openapi_config()).await {
            Ok(index_list) => Ok(index_list),
            Err(e) => Err(PineconeError::ListIndexesError { openapi_error: e }),
        }
    }

    /// Creates a pod index.
    ///
    /// ### Arguments
    /// * `name: String` - The name of the index
    /// * `dimension: i32` - The dimension of the index
    /// * `metric: Metric` - The metric to use for the index
    /// * `environment: String` - The environment where the pod index will be deployed. Example: 'us-east1-gcp'
    /// * `pod_type: String` - This value combines pod type and pod size into a single string. This configuration is your main lever for vertical scaling.
    /// * `pods: i32` - The number of pods to deploy. Default: 1
    /// * `replicas: Option<i32>` - The number of replicas to deploy for the pod index. Default: 1
    /// * `shards: Option<i32>` - The number of shards to use. Shards are used to expand the amount of vectors you can store beyond the capacity of a single pod. Default: 1
    /// * `metadata_indexed: Option<Vec<String>>` - The metadata fields to index.
    /// * `source_collection: Option<String>` - The name of the collection to use as the source for the pod index. This configuration is only used when creating a pod index from an existing collection.
    ///
    /// ### Return
    /// * Returns a `Result<IndexModel, PineconeError>` object.
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::utils::errors::PineconeError;
    /// use pinecone_sdk::control::{Metric, Cloud, IndexModel};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError> {
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// // Create a pod index.
    /// let create_index_response: Result<IndexModel, PineconeError> = pinecone.create_pod_index(
    ///     "index_name", // Name of the index
    ///     10, // Dimension of the index
    ///     Metric::Cosine, // Distance metric
    ///     "us-east-1-aws", // Environment
    ///     "p1.x1", // Pod type
    ///     1, // Number of pods
    ///     Some(1), // Number of replicas
    ///     Some(1), // Number of shards
    ///     Some( // Metadata fields to index
    ///         &vec!["genre",
    ///         "title",
    ///         "imdb_rating"]),
    ///     Some("example-collection"), // Source collection
    /// )
    /// .await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_pod_index(
        &self,
        name: &str,
        dimension: i32,
        metric: Metric,
        environment: &str,
        pod_type: &str,
        pods: i32,
        replicas: Option<i32>,
        shards: Option<i32>,
        metadata_indexed: Option<&[&str]>,
        source_collection: Option<&str>,
    ) -> Result<IndexModel, PineconeError> {
        let indexed = metadata_indexed.map(|i| i.iter().map(|s| s.to_string()).collect());

        let pod_spec = PodSpec {
            environment: environment.to_string(),
            replicas,
            shards,
            pod_type: pod_type.to_string(),
            pods,
            metadata_config: Some(Box::new(PodSpecMetadataConfig { indexed })),
            source_collection: source_collection.map(|s| s.to_string()),
        };

        let spec = CreateIndexRequestSpec {
            serverless: None,
            pod: Some(Box::new(pod_spec)),
        };

        let create_index_request = CreateIndexRequest {
            name: name.to_string(),
            dimension,
            metric: Some(metric),
            spec: Some(Box::new(spec)),
        };

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
    /// let delete_index_response: Result<(), PineconeError> = pinecone.delete_index("index-name").await;
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
    /// use pinecone_sdk::control::CollectionModel;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// // Describe an index in the project.
    /// let create_collection_response: Result<CollectionModel, PineconeError> = pinecone.create_collection("collection-name", "index-name").await;
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

    /// Lists all collections.
    ///
    /// This operation returns a list of all collections in a project.
    ///
    /// ### Return
    /// * `Result<CollectionList, PineconeError>`
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
    /// // List all collections in the project.
    /// let collection_list = pinecone.list_collections().await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_collections(&self) -> Result<CollectionList, PineconeError> {
        match manage_indexes_api::list_collections(&self.openapi_config()).await {
            Ok(collection_list) => Ok(collection_list),
            Err(e) => Err(PineconeError::ListCollectionsError { openapi_error: e }),
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
                "p1.x1",
                1,
                Some(1),
                Some(1),
                Some(&vec!["genre", "title", "imdb_rating"]),
                Some("example-collection"),
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
                "p1.x1",
                1,
                None,
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
    async fn test_create_pod_index_invalid_environment() -> Result<(), PineconeError> {
        let _m = mock("POST", "/indexes")
            .with_status(400)
            .with_header("content-type", "application/json")
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
                "invalid-environment",
                "p1.x1",
                1,
                Some(1),
                Some(1),
                Some(&vec!["genre", "title", "imdb_rating"]),
                Some("example-collection"),
            )
            .await
            .expect_err("Expected create_pod_index to return an error");

        assert!(matches!(
            create_index_response,
            PineconeError::CreateIndexError { .. }
        ));

        Ok(())
    }

    #[tokio::test]
    async fn test_create_pod_index_invalid_pod_type() -> Result<(), PineconeError> {
        let _m = mock("POST", "/indexes")
            .with_status(400)
            .with_header("content-type", "application/json")
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
                "invalid-pod-type",
                1,
                Some(1),
                Some(1),
                Some(&vec!["genre", "title", "imdb_rating"]),
                Some("example-collection"),
            )
            .await
            .expect_err("Expected create_pod_index to return an error");

        assert!(matches!(
            create_index_response,
            PineconeError::CreateIndexError { .. }
        ));

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

    #[tokio::test]
    async fn test_list_collections() -> Result<(), PineconeError> {
        // Create a mock server
        let _m = mock("GET", "/collections")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "collections": [
                      {
                        "name": "small-collection",
                        "size": 3126700,
                        "status": "Ready",
                        "dimension": 3,
                        "vector_count": 99,
                        "environment": "us-east1-gcp"
                      },
                      {
                        "name": "small-collection-new",
                        "size": 3126700,
                        "status": "Initializing",
                        "dimension": 3,
                        "vector_count": 99,
                        "environment": "us-east1-gcp"
                      },
                      {
                        "name": "big-collection",
                        "size": 160087040000000,
                        "status": "Ready",
                        "dimension": 1536,
                        "vector_count": 10000000,
                        "environment": "us-east1-gcp"
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

        // Call list_collections and verify the result
        let collection_list = pinecone
            .list_collections()
            .await
            .expect("Failed to list collections");

        let expected = CollectionList {
            // name: String, dimension: i32, metric: Metric, host: String, spec: models::IndexModelSpec, status: models::IndexModelStatus)
            collections: Some(vec![
                CollectionModel {
                    name: "small-collection".to_string(),
                    size: Some(3126700),
                    status: Status::Ready,
                    dimension: Some(3),
                    vector_count: Some(99),
                    environment: "us-east1-gcp".to_string(),
                },
                CollectionModel {
                    name: "small-collection-new".to_string(),
                    size: Some(3126700),
                    status: Status::Initializing,
                    dimension: Some(3),
                    vector_count: Some(99),
                    environment: "us-east1-gcp".to_string(),
                },
                CollectionModel {
                    name: "big-collection".to_string(),
                    size: Some(160087040000000),
                    status: Status::Ready,
                    dimension: Some(1536),
                    vector_count: Some(10000000),
                    environment: "us-east1-gcp".to_string(),
                },
            ]),
        };
        assert_eq!(collection_list, expected);

        Ok(())
    }
}
