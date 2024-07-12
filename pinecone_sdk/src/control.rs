use std::cmp::min;
use std::time::Duration;

use crate::pinecone::PineconeClient;
use crate::utils::errors::PineconeError;

pub use openapi::apis::manage_indexes_api::{
    self, ConfigureIndexError, CreateCollectionError, CreateIndexError, DeleteCollectionError,
    DeleteIndexError, DescribeCollectionError, DescribeIndexError, ListCollectionsError,
    ListIndexesError,
};
pub use openapi::models::create_index_request::Metric;
pub use openapi::models::serverless_spec::Cloud;
pub use openapi::models::{
    CollectionList, CollectionModel, ConfigureIndexRequest, ConfigureIndexRequestSpec,
    ConfigureIndexRequestSpecPod, CreateCollectionRequest, CreateIndexRequest, IndexList,
    IndexModel, IndexSpec, PodSpec, PodSpecMetadataConfig, ServerlessSpec,
};

/// Defines the wait policy for index creation.
#[derive(Debug)]
pub enum WaitPolicy {
    /// Wait for the index to become ready, up to the specified duration.
    WaitFor(Duration),

    /// Do not wait for the index to become ready -- return immediately.
    NoWait,
}

impl Default for WaitPolicy {
    fn default() -> Self {
        WaitPolicy::WaitFor(Duration::from_secs(300))
    }
}

impl PineconeClient {
    /// Creates a serverless index.
    ///
    /// ### Arguments
    /// * `name: &str` - Name of the index to create.
    /// * `dimension: i32` - Dimension of the vectors to be inserted in the index.
    /// * `metric: Metric` - The distance metric to be used for similarity search.
    /// * `cloud: Cloud` - The public cloud where you would like your index hosted.
    /// * `region: &str` - The region where you would like your index to be created.
    /// * `timeout: WaitPolicy` - The wait policy for index creation. If the index becomes ready before the specified duration, the function will return early. If the index is not ready after the specified duration, the function will return an error.
    ///
    /// ### Return
    /// * `Result<IndexModel, PineconeError>`
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::utils::errors::PineconeError;
    /// use pinecone_sdk::control::{Metric, Cloud, WaitPolicy, IndexModel, CreateIndexError};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError<CreateIndexError>>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// // Create an index.
    /// let create_index_response: Result<IndexModel, PineconeError<CreateIndexError>> = pinecone.create_serverless_index(
    ///     "index-name", // Name of the index
    ///     10, // Dimension of the vectors
    ///     Metric::Cosine, // Distance metric
    ///     Cloud::Aws, // Cloud provider
    ///     "us-east-1", // Region
    ///     WaitPolicy::NoWait // Timeout
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
        timeout: WaitPolicy,
    ) -> Result<IndexModel, PineconeError<CreateIndexError>> {
        // create request specs
        let create_index_request_spec = IndexSpec {
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

        // make openAPI call
        let res = manage_indexes_api::create_index(&self.openapi_config(), create_index_request)
            .await
            .map_err(|e| PineconeError::from((e, format!("Failed to create index \"{name}\""))))?;

        // poll index status
        match self.handle_poll_index(name, timeout).await {
            Ok(_) => Ok(res),
            Err(e) => Err(e),
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
    /// * `timeout: WaitPolicy` - The wait policy for index creation. If the index becomes ready before the specified duration, the function will return early. If the index is not ready after the specified duration, the function will return an error.
    ///
    /// ### Return
    /// * Returns a `Result<IndexModel, PineconeError<CreateIndexError>>` object.
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::utils::errors::PineconeError;
    /// use pinecone_sdk::control::{Metric, Cloud, WaitPolicy, IndexModel, CreateIndexError};
    /// use std::time::Duration;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError<CreateIndexError>> {
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// // Create a pod index.
    /// let create_index_response: Result<IndexModel, PineconeError<CreateIndexError>> = pinecone.create_pod_index(
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
    ///     WaitPolicy::WaitFor(Duration::from_secs(10)), // Timeout
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
        timeout: WaitPolicy,
    ) -> Result<IndexModel, PineconeError<CreateIndexError>> {
        // create request specs
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

        let spec = IndexSpec {
            serverless: None,
            pod: Some(Box::new(pod_spec)),
        };

        let create_index_request = CreateIndexRequest {
            name: name.to_string(),
            dimension,
            metric: Some(metric),
            spec: Some(Box::new(spec)),
        };

        // make openAPI call
        let res = manage_indexes_api::create_index(&self.openapi_config(), create_index_request)
            .await
            .map_err(|e| PineconeError::from((e, format!("Failed to create index \"{name}\""))))?;

        // poll index status
        match self.handle_poll_index(name, timeout).await {
            Ok(_) => Ok(res),
            Err(e) => Err(e),
        }
    }

    // Checks if the index is ready by polling the index status
    async fn handle_poll_index(
        &self,
        name: &str,
        timeout: WaitPolicy,
    ) -> Result<(), PineconeError<CreateIndexError>> {
        match timeout {
            WaitPolicy::WaitFor(duration) => {
                let start_time = std::time::Instant::now();

                loop {
                    // poll index status, if ready return early
                    if self.is_ready(name).await {
                        break;
                    }

                    match duration.cmp(&start_time.elapsed()) {
                        // if index not ready after waiting specified duration, return error
                        std::cmp::Ordering::Less => {
                            let message = format!("Index \"{name}\" not ready");
                            return Err(PineconeError::TimeoutError { message });
                        }
                        // if still waiting, sleep for 5 seconds or remaining time
                        std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                            let time_remaining = duration.saturating_sub(start_time.elapsed());
                            tokio::time::sleep(Duration::from_millis(min(
                                time_remaining.as_millis() as u64,
                                5000,
                            )))
                            .await;
                        }
                    }
                }
            }
            WaitPolicy::NoWait => {}
        }

        Ok(())
    }

    // Gets ready status of an index
    async fn is_ready(&self, name: &str) -> bool {
        let res = manage_indexes_api::describe_index(&self.openapi_config(), name).await;
        match res {
            Ok(index) => index.status.ready,
            Err(_) => false,
        }
    }

    /// Describes an index.
    ///
    /// ### Arguments
    /// * `name: &str` - Name of the index to describe.
    ///
    /// ### Return
    /// * `Result<IndexModel, PineconeError<DescribeIndexError>>`
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::utils::errors::PineconeError;
    /// use pinecone_sdk::control::{IndexModel, DescribeIndexError};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError<DescribeIndexError>>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// // Describe an index in the project.
    /// let describe_index_response: Result<IndexModel, PineconeError<DescribeIndexError>> = pinecone.describe_index("index-name").await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn describe_index(
        &self,
        name: &str,
    ) -> Result<IndexModel, PineconeError<DescribeIndexError>> {
        // make openAPI call
        let res = manage_indexes_api::describe_index(&self.openapi_config(), name)
            .await
            .map_err(|e| {
                PineconeError::from((e, format!("Failed to describe index \"{name}\"")))
            })?;

        Ok(res)
    }

    /// Lists all indexes.
    ///
    /// The results include a description of all indexes in your project, including the
    /// index name, dimension, metric, status, and spec.
    ///
    /// ### Return
    /// * `Result<IndexList, PineconeError<ListIndexesError>>`
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::utils::errors::PineconeError;
    /// use pinecone_sdk::control::{IndexList, ListIndexesError};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError<ListIndexesError>>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// // List all indexes in the project.
    /// let index_list_response: Result<IndexList, PineconeError<ListIndexesError>> = pinecone.list_indexes().await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_indexes(&self) -> Result<IndexList, PineconeError<ListIndexesError>> {
        // make openAPI call
        let res = manage_indexes_api::list_indexes(&self.openapi_config())
            .await
            .map_err(|e| PineconeError::from((e, format!("Failed to list indexes"))))?;

        Ok(res)
    }

    /// Configures an index.
    ///
    /// This operation specifies the pod type and number of replicas for an index.
    /// It applies to pod-based indexes only.
    /// Serverless indexes scale automatically based on usage.
    ///
    /// ### Arguments
    /// * name: &str - The name of the index to be configured.
    /// * replicas: i32 - The desired number of replicas, lowest value is 0.
    /// * pod_type: &str - the new pod_type for the index. To learn more about the available pod types, please see [Understanding Indexes](https://docs.pinecone.io/docs/indexes)
    ///
    /// ### Return
    /// * `Result<IndexModel, PineconeError<ConfigureIndexError>>`
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::utils::errors::PineconeError;
    /// use pinecone_sdk::control::{IndexModel, ConfigureIndexError};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError<ConfigureIndexError>>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// let response: Result<IndexModel, PineconeError<ConfigureIndexError>> = pinecone.configure_index("index-name", 6, "s1").await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn configure_index(
        &self,
        name: &str,
        replicas: i32,
        pod_type: &str,
    ) -> Result<IndexModel, PineconeError<ConfigureIndexError>> {
        let configure_index_request = ConfigureIndexRequest::new(ConfigureIndexRequestSpec::new(
            ConfigureIndexRequestSpecPod {
                replicas: Some(replicas),
                pod_type: Some(pod_type.to_string()),
            },
        ));

        // make openAPI call
        let res = manage_indexes_api::configure_index(
            &self.openapi_config(),
            name,
            configure_index_request,
        )
        .await
        .map_err(|e| PineconeError::from((e, format!("Failed to configure index \"{name}\""))))?;

        Ok(res)
    }

    /// Deletes an index.
    ///
    /// ### Arguments
    /// * name: &str - The name of the index to be deleted.
    ///
    /// ### Return
    /// * Returns a `Result<(), PineconeError<DeleteIndexError>>` object.
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::utils::errors::PineconeError;
    /// use pinecone_sdk::control::DeleteIndexError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError<DeleteIndexError>>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// let delete_index_response: Result<(), PineconeError<DeleteIndexError>> = pinecone.delete_index("index-name").await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_index(&self, name: &str) -> Result<(), PineconeError<DeleteIndexError>> {
        // make openAPI call
        let res = manage_indexes_api::delete_index(&self.openapi_config(), name)
            .await
            .map_err(|e| PineconeError::from((e, format!("Failed to delete index \"{name}\""))))?;

        Ok(res)
    }

    /// Creates a collection from an index.
    ///
    /// ### Arguments
    /// * `name: &str` - Name of the collection to create.
    /// * `source: &str` - Name of the index to be used as the source for the collection.
    ///
    /// ### Return
    /// * `Result<CollectionModel, PineconeError<CreateCollectionError>>`
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::utils::errors::PineconeError;
    /// use pinecone_sdk::control::{CollectionModel, CreateCollectionError};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError<CreateCollectionError>>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// // Describe an index in the project.
    /// let create_collection_response: Result<CollectionModel, PineconeError<CreateCollectionError>> = pinecone.create_collection("collection-name", "index-name").await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_collection(
        &self,
        name: &str,
        source: &str,
    ) -> Result<CollectionModel, PineconeError<CreateCollectionError>> {
        let create_collection_request = CreateCollectionRequest {
            name: name.to_string(),
            source: source.to_string(),
        };

        // make openAPI call
        let res = manage_indexes_api::create_collection(
            &self.openapi_config(),
            create_collection_request,
        )
        .await
        .map_err(|e| PineconeError::from((e, format!("Failed to create collection \"{name}\""))))?;

        Ok(res)
    }

    /// Describe a collection.
    ///
    /// ### Arguments
    /// * name: &str - The name of the collection to describe.
    ///
    /// ### Return
    /// * Returns a `Result<(), PineconeError<DescribeCollectionError>>` object.
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::utils::errors::PineconeError;
    /// use pinecone_sdk::control::{CollectionModel, DescribeCollectionError};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError<DescribeCollectionError>>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// let response: Result<CollectionModel, PineconeError<DescribeCollectionError>> = pinecone.describe_collection("collection-name").await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn describe_collection(
        &self,
        name: &str,
    ) -> Result<CollectionModel, PineconeError<DescribeCollectionError>> {
        let res = manage_indexes_api::describe_collection(&self.openapi_config(), name)
            .await
            .map_err(|e| {
                PineconeError::from((e, format!("Failed to describe collection \"{name}\"")))
            })?;

        Ok(res)
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
    /// use pinecone_sdk::control::{CollectionList, ListCollectionsError};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError<ListCollectionsError>>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// // List all collections in the project.
    /// let collection_list:Result<CollectionList, PineconeError<ListCollectionsError>> = pinecone.list_collections().await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_collections(
        &self,
    ) -> Result<CollectionList, PineconeError<ListCollectionsError>> {
        // make openAPI call
        let res = manage_indexes_api::list_collections(&self.openapi_config())
            .await
            .map_err(|e| PineconeError::from((e, format!("Failed to list collections"))))?;

        Ok(res)
    }

    /// Deletes a collection.
    ///
    /// ### Arguments
    /// * name: &str - The name of the collection to be deleted.
    ///
    /// ### Return
    /// * Returns a `Result<(), PineconeError<DeleteCollectionError>>` object.
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::utils::errors::PineconeError;
    /// use pinecone_sdk::control::DeleteCollectionError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError<DeleteCollectionError>>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// let response: Result<(), PineconeError<DeleteCollectionError>> = pinecone.delete_collection("collection-name").await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_collection(
        &self,
        name: &str,
    ) -> Result<(), PineconeError<DeleteCollectionError>> {
        // make openAPI call
        let res = manage_indexes_api::delete_collection(&self.openapi_config(), name)
            .await
            .map_err(|e| {
                PineconeError::from((e, format!("Failed to delete collection \"{name}\"")))
            })?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use manage_indexes_api::CreateIndexError;
    use openapi::models::{self, collection_model::Status, IndexList};
    use tokio;

    #[tokio::test]
    async fn test_create_serverless_index() -> Result<(), PineconeError<CreateIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/indexes");
            then.status(201)
                .header("content-type", "application/json")
                .body(
                    r#"
                {
                    "name": "index-name",
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
                }"#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_serverless_index(
                "index-name",
                10,
                Metric::Cosine,
                Cloud::Aws,
                "us-east-1",
                WaitPolicy::NoWait,
            )
            .await
            .expect("Failed to create serverless index");

        mock.assert();

        assert_eq!(create_index_response.name, "index-name");
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
    async fn test_create_serverless_index_defaults() -> Result<(), PineconeError<CreateIndexError>>
    {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/indexes");
            then.status(201)
                .header("content-type", "application/json")
                .body(
                    r#"{
                    "name": "index-name",
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
                }"#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_serverless_index(
                "index-name",
                10,
                Default::default(),
                Default::default(),
                "us-east-1",
                WaitPolicy::NoWait,
            )
            .await
            .expect("Failed to create serverless index");

        assert_eq!(create_index_response.name, "index-name");
        assert_eq!(create_index_response.dimension, 10);
        assert_eq!(
            create_index_response.metric,
            openapi::models::index_model::Metric::Cosine
        );

        let spec = create_index_response.spec.serverless.unwrap();
        assert_eq!(spec.cloud, openapi::models::serverless_spec::Cloud::Gcp);
        assert_eq!(spec.region, "us-east-1");

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_create_serverless_index_invalid_region(
    ) -> Result<(), PineconeError<CreateIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/indexes");
            then.status(404)
                .header("content-type", "application/json")
                .body(
                    r#"{
                    "error": {
                        "code": "NOT_FOUND",
                        "message": "Resource cloud: aws region: abc not found."
                    },
                    "status": 404
                }"#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_serverless_index(
                "index-name",
                10,
                Default::default(),
                Default::default(),
                "abc",
                WaitPolicy::NoWait,
            )
            .await
            .expect_err("Expected error when creating serverless index");

        assert!(matches!(
            create_index_response,
            PineconeError::InvalidRegionError { .. }
        ));
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_create_serverless_index_index_exists(
    ) -> Result<(), PineconeError<CreateIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/indexes");
            then.status(409)
                .header("content-type", "application/json")
                .body(
                    r#"{
                        "error": {
                            "code": "ALREADY_EXISTS",
                            "message": "Resource already exists."
                        },
                        "status": 409
                    }"#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_serverless_index(
                "index-name",
                10,
                Default::default(),
                Default::default(),
                "us-west-1",
                WaitPolicy::NoWait,
            )
            .await
            .expect_err("Expected error when creating serverless index");

        assert!(matches!(
            create_index_response,
            PineconeError::IndexAlreadyExistsError { .. }
        ));
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_create_serverless_index_unprocessable_entity(
    ) -> Result<(), PineconeError<CreateIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/indexes");
            then.status(422)
                .header("content-type", "application/json")
                .body(
                r#"{
                    "error": {
                            "code": "INVALID_ARGUMENT",
                            "message": "Failed to deserialize the JSON body into the target type: missing field `metric` at line 1 column 16"
                        },
                    "status": 422
                }"#,
            );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_serverless_index(
                "index-name",
                10,
                Default::default(),
                Default::default(),
                "us-west-1",
                WaitPolicy::NoWait,
            )
            .await
            .expect_err("Expected error when creating serverless index");

        assert!(matches!(
            create_index_response,
            PineconeError::UnprocessableEntityError { .. }
        ));
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_create_serverless_index_internal_error(
    ) -> Result<(), PineconeError<CreateIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/indexes");
            then.status(500);
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_serverless_index(
                "index-name",
                10,
                Metric::Cosine,
                Cloud::Aws,
                "us-east-1",
                WaitPolicy::NoWait,
            )
            .await
            .expect_err("Expected create_index to return an error");

        assert!(matches!(
            create_index_response,
            PineconeError::InternalServerError { .. }
        ));
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_describe_serverless_index() -> Result<(), PineconeError<DescribeIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/indexes/serverless-index");
            then.status(200)
                .header("content-type", "application/json")
                .body(
                    r#"{
                    "dimension": 1536,
                    "host": "mock-host",
                    "metric": "cosine",
                    "name": "serverless-index",
                    "spec": {
                        "serverless": {
                        "cloud": "aws",
                        "region": "us-east-1"
                        }
                    },
                    "status": {
                        "ready": true,
                        "state": "Ready"
                    }
                }"#,
                );
        });

        // Construct Pinecone instance with the mock server URL
        let pinecone = PineconeClient::new(
            Some("api-key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
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
            host: "mock-host".to_string(),
            spec: Box::new(models::IndexModelSpec {
                serverless: Some(Box::new(models::ServerlessSpec {
                    cloud: openapi::models::serverless_spec::Cloud::Aws,
                    region: "us-east-1".to_string(),
                })),
                pod: None,
            }),
        };

        assert_eq!(index, expected);
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_describe_index_invalid_name() -> Result<(), PineconeError<DescribeIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/indexes/invalid-index");
            then.status(404)
                .header("content-type", "application/json")
                .body(
                    r#"{
                    "error": "Index invalid-index not found"
                }"#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let describe_index_response = pinecone
            .describe_index("invalid-index")
            .await
            .expect_err("Expected describe_index to return an error");

        assert!(matches!(
            describe_index_response,
            PineconeError::IndexNotFoundError { .. }
        ));
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_describe_index_server_error() -> Result<(), PineconeError<DescribeIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/indexes/index-name");
            then.status(500);
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let describe_index_response = pinecone
            .describe_index("index-name")
            .await
            .expect_err("Expected describe_index to return an error");

        assert!(matches!(
            describe_index_response,
            PineconeError::InternalServerError { .. }
        ));
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_list_indexes() -> Result<(), PineconeError<ListIndexesError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/indexes");
            then.status(200)
                .header("content-type", "application/json")
                .body(
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
                }"#,
                );
        });

        // Construct Pinecone instance with the mock server URL
        let pinecone = PineconeClient::new(
            Some("api-key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
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
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_list_indexes_server_error() -> Result<(), PineconeError<ListIndexesError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/indexes");
            then.status(500);
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let list_indexes_response = pinecone
            .list_indexes()
            .await
            .expect_err("Expected list_indexes to return an error");

        assert!(matches!(
            list_indexes_response,
            PineconeError::InternalServerError { .. }
        ));
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_create_pod_index() -> Result<(), PineconeError<CreateIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/indexes");
            then.status(201)
                .header("content-type", "application/json")
                .body(
                    r#"
                {
                    "name": "index-name",
                    "dimension": 1536,
                    "metric": "euclidean",
                    "host": "mock-host",
                    "spec": {
                        "pod": {
                            "environment": "us-east-1-aws",
                            "replicas": 1,
                            "shards": 1,
                            "pod_type": "p1.x1",
                            "pods": 1,
                            "metadata_config": {
                                "indexed": [
                                    "genre",
                                    "title",
                                    "imdb_rating"
                                ]
                            }
                        }
                    },
                    "status": {
                        "ready": true,
                        "state": "ScalingUpPodSize"
                    }
                }
            "#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_pod_index(
                "index-name",
                1536,
                Metric::Euclidean,
                "us-east-1-aws",
                "p1.x1",
                1,
                Some(1),
                Some(1),
                Some(&vec!["genre", "title", "imdb_rating"]),
                Some("example-collection"),
                WaitPolicy::NoWait,
            )
            .await
            .expect("Failed to create pod index");

        assert_eq!(create_index_response.name, "index-name");
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

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_create_pod_index_with_defaults() -> Result<(), PineconeError<CreateIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/indexes");
            then.status(201)
                .header("content-type", "application/json")
                .body(
                    r#"
                {
                    "name": "index-name",
                    "dimension": 1536,
                    "metric": "cosine",
                    "host": "mock-host",
                    "spec": {
                        "pod": {
                            "environment": "us-east-1-aws",
                            "pod_type": "p1.x1",
                            "pods": 1,
                            "metadata_config": {},
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
                );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_pod_index(
                "index-name",
                1536,
                Default::default(),
                "us-east-1-aws",
                "p1.x1",
                1,
                None,
                None,
                None,
                None,
                WaitPolicy::NoWait,
            )
            .await
            .expect("Failed to create pod index");

        assert_eq!(create_index_response.name, "index-name");
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

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_create_pod_index_quota_exceeded() -> Result<(), PineconeError<CreateIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/indexes");
            then.status(403)
                .header("content-type", "application/json")
                .body(
                    r#"
                    {
                        "error": {
                            "code": "FORBIDDEN",
                            "message": "Increase yoru quota or upgrade to create more indexes."
                        },
                        "status": 403
                    }
                "#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_pod_index(
                "index-name",
                1536,
                Metric::Euclidean,
                "test-environment",
                "p1.x1",
                1,
                Some(1),
                Some(1),
                None,
                Some("example-collection"),
                WaitPolicy::NoWait,
            )
            .await
            .expect_err("Expected create_pod_index to return an error");

        assert!(matches!(
            create_index_response,
            PineconeError::PodQuotaExceededError { .. }
        ));

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_create_pod_index_invalid_environment(
    ) -> Result<(), PineconeError<CreateIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/indexes");
            then.status(400)
                .header("content-type", "application/json")
                .body(
                    r#"
                    {
                        "error": "Invalid environment"
                    }
                "#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_pod_index(
                "index-name",
                1536,
                Metric::Euclidean,
                "invalid-environment",
                "p1.x1",
                1,
                Some(1),
                Some(1),
                Some(&vec!["genre", "title", "imdb_rating"]),
                Some("example-collection"),
                WaitPolicy::NoWait,
            )
            .await
            .expect_err("Expected create_pod_index to return an error");

        assert!(matches!(
            create_index_response,
            PineconeError::BadRequestError { .. }
        ));

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_create_pod_index_invalid_pod_type() -> Result<(), PineconeError<CreateIndexError>>
    {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/indexes");
            then.status(400)
                .header("content-type", "application/json")
                .body(
                    r#"
                    {
                        "error": "Invalid pod type"
                    }
                "#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_pod_index(
                "index-name",
                1536,
                Metric::Euclidean,
                "us-east-1-aws",
                "invalid-pod-type",
                1,
                Some(1),
                Some(1),
                Some(&vec!["genre", "title", "imdb_rating"]),
                Some("example-collection"),
                WaitPolicy::NoWait,
            )
            .await
            .expect_err("Expected create_pod_index to return an error");

        assert!(matches!(
            create_index_response,
            PineconeError::BadRequestError { .. }
        ));
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_handle_polling_index_ok() -> Result<(), PineconeError<CreateIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/indexes/index-name");
            then.status(200)
                .header("content-type", "application/json")
                .body(
                    r#"
                {
                    "dimension": 1536,
                    "host": "mock-host",
                    "metric": "cosine",
                    "name": "index-name",
                    "spec": {
                        "serverless": {
                        "cloud": "aws",
                        "region": "us-east-1"
                        }
                    },
                    "status": {
                        "ready": true,
                        "state": "Ready"
                    }
                }"#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api-key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let res = pinecone
            .handle_poll_index("index-name", WaitPolicy::WaitFor(Duration::from_secs(1)))
            .await;

        assert!(res.is_ok());
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_handle_polling_index_err() -> Result<(), PineconeError<CreateIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/indexes/index-name");
            then.status(200)
                .header("content-type", "application/json")
                .body(
                    r#"
                    {
                        "dimension": 1536,
                        "host": "mock-host",
                        "metric": "cosine",
                        "name": "index-name",
                        "spec": {
                            "serverless": {
                            "cloud": "aws",
                            "region": "us-east-1"
                            }
                        },
                        "status": {
                            "ready": false,
                            "state": "Initializing"
                        }
                    }"#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api-key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let start_time = std::time::Instant::now();
        let err = pinecone
            .handle_poll_index("index-name", WaitPolicy::WaitFor(Duration::from_secs(7)))
            .await
            .expect_err("Expected to fail polling index");

        assert!(start_time.elapsed().as_secs() >= 7 && start_time.elapsed().as_secs() < 8);
        assert!(matches!(err, PineconeError::TimeoutError { .. }));

        mock.assert_hits(3);

        Ok(())
    }

    #[tokio::test]
    async fn test_configure_index() -> Result<(), PineconeError<ConfigureIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.path("/indexes/index-name");
            then.status(202)
                .header("content-type", "application/json")
                .body(
                    r#"
                {
                    "name": "index-name",
                    "dimension": 1536,
                    "metric": "cosine",
                    "host": "mock-host",
                    "spec": {
                        "pod": {
                            "environment": "us-east-1-aws",
                            "replicas": 6,
                            "shards": 1,
                            "pod_type": "p1.x1",
                            "pods": 1,
                            "metadata_config": {
                                "indexed": [
                                    "genre",
                                    "title",
                                    "imdb_rating"
                                ]
                            }
                        }
                    },
                    "status": {
                        "ready": true,
                        "state": "ScalingUpPodSize"
                    }
                }"#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let configure_index_response = pinecone
            .configure_index("index-name", 6, "p1.x1")
            .await
            .expect("Failed to configure index");

        assert_eq!(configure_index_response.name, "index-name");

        let spec = configure_index_response.spec.pod.unwrap();
        assert_eq!(spec.replicas.unwrap(), 6);
        assert_eq!(spec.pod_type.as_str(), "p1.x1");

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_configure_index_quota_exceeded() -> Result<(), PineconeError<ConfigureIndexError>>
    {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.path("/indexes/index-name");
            then.status(403)
                .header("content-type", "application/json")
                .body(
                    r#"
                    {
                        "error": {
                            "code": "FORBIDDEN",
                            "message": "Increase your quota or upgrade to create more indexes."
                        },
                        "status": 403
                    }
                "#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let configure_index_response = pinecone
            .configure_index("index-name", 6, "p1.x1")
            .await
            .expect_err("Expected to fail to configure index");

        assert!(matches!(
            configure_index_response,
            PineconeError::PodQuotaExceededError { .. }
        ));

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_configure_index_not_found() -> Result<(), PineconeError<ConfigureIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.path("/indexes/index-name");
            then.status(404)
                .header("content-type", "application/json")
                .body(
                    r#"{
                    "error": {
                        "code": "NOT_FOUND",
                        "message": "Index index-name not found."
                    },
                    "status": 404
                }"#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let configure_index_response = pinecone
            .configure_index("index-name", 6, "p1.x1")
            .await
            .expect_err("Expected to fail to configure index");

        assert!(matches!(
            configure_index_response,
            PineconeError::IndexNotFoundError { .. }
        ));

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_configure_index_unprocessable_entity(
    ) -> Result<(), PineconeError<ConfigureIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.path("/indexes/index-name");
            then.status(422)
                .header("content-type", "application/json")
                .body(
                    r#"{
                    "error": {
                            "code": "INVALID_ARGUMENT",
                            "message": "Failed to deserialize the JSON body into the target type
                        },
                    "status": 422
                }"#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let configure_index_response = pinecone
            .configure_index("index-name", 6, "p1.x1")
            .await
            .expect_err("Expected to fail to configure index");

        assert!(matches!(
            configure_index_response,
            PineconeError::UnprocessableEntityError { .. }
        ));

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_configure_index_internal_error() -> Result<(), PineconeError<ConfigureIndexError>>
    {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.path("/indexes/index-name");
            then.status(500);
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let configure_index_response = pinecone
            .configure_index("index-name", 6, "p1.x1")
            .await
            .expect_err("Expected to fail to configure index");
        eprintln!("hi");

        assert!(matches!(
            configure_index_response,
            PineconeError::InternalServerError { .. }
        ));
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_index() -> Result<(), PineconeError<DeleteIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(DELETE).path("/indexes/index-name");
            then.status(202);
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let _ = pinecone
            .delete_index("index-name")
            .await
            .expect("Failed to delete index");

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_index_invalid_name() -> Result<(), PineconeError<DeleteIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(DELETE).path("/indexes/invalid-index");
            then.status(404)
                .header("content-type", "application/json")
                .body(
                    r#"
                    {
                        "error": "Index not found"
                    }
                "#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let delete_index_response = pinecone
            .delete_index("invalid-index")
            .await
            .expect_err("Expected delete_index to return an error");

        assert!(matches!(
            delete_index_response,
            PineconeError::IndexNotFoundError { .. }
        ));

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_index_pending_collection() -> Result<(), PineconeError<DeleteIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(DELETE).path("/indexes/index-name");
            then.status(412);
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let delete_index_response = pinecone
            .delete_index("index-name")
            .await
            .expect_err("Expected delete_index to return an error");

        assert!(matches!(
            delete_index_response,
            PineconeError::PendingCollectionError { .. }
        ));

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_index_server_error() -> Result<(), PineconeError<DeleteIndexError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(DELETE).path("/indexes/index-name");
            then.status(500);
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let delete_index_response = pinecone
            .delete_index("index-name")
            .await
            .expect_err("Expected delete_index to return an error");

        assert!(matches!(
            delete_index_response,
            PineconeError::InternalServerError { .. }
        ));

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_create_collection() -> Result<(), PineconeError<CreateCollectionError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/collections");
            then.status(201)
                .header("content-type", "application/json")
                .body(
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
                );
        });

        // Construct Pinecone instance with the mock server URL
        let pinecone = PineconeClient::new(
            Some("api-key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
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

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_create_collection_quota_exceeded(
    ) -> Result<(), PineconeError<CreateCollectionError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/collections");
            then.status(403)
                .header("content-type", "application/json")
                .body(
                    r#"
                    {
                        "error": {
                            "code": "FORBIDDEN",
                            "message": "Collection exceeds quota. Maximum allowed on your account is 1. Currently have 1."
                        },
                        "status": 403
                    }
                "#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let create_collection_response = pinecone
            .create_collection("invalid_collection", "valid-index")
            .await
            .expect_err("Expected create_collection to return an error");

        assert!(matches!(
            create_collection_response,
            PineconeError::CollectionsQuotaExceededError { .. }
        ));

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_create_collection_invalid_name(
    ) -> Result<(), PineconeError<CreateCollectionError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/collections");
            then.status(409)
                .header("content-type", "application/json")
                .body(
                    r#"
                    {
                        "error": "Index not found"
                    }
                "#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let create_collection_response = pinecone
            .create_collection("invalid_collection", "valid-index")
            .await
            .expect_err("Expected create_collection to return an error");

        assert!(matches!(
            create_collection_response,
            PineconeError::CollectionAlreadyExistsError { .. }
        ));

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_create_collection_server_error(
    ) -> Result<(), PineconeError<CreateCollectionError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/collections");
            then.status(500);
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let create_collection_response = pinecone
            .create_collection("collection-name", "index1")
            .await
            .expect_err("Expected create_collection to return an error");

        assert!(matches!(
            create_collection_response,
            PineconeError::InternalServerError { .. }
        ));

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_describe_collection() -> Result<(), PineconeError<DescribeCollectionError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/collections/collection-name");
            then.status(200)
                .header("content-type", "application/json")
                .body(
                    r#"{
                        "dimension": 3,
                        "environment": "us-east1-gcp",
                        "name": "tiny-collection",
                        "size": 3126700,
                        "status": "Ready",
                        "vector_count": 99
                      }"#,
                );
        });

        // Construct Pinecone instance with the mock server URL
        let pinecone = PineconeClient::new(
            Some("api-key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        // Call describe_collection and verify the result
        let collection = pinecone
            .describe_collection("collection-name")
            .await
            .expect("Failed to describe collection");

        let expected = CollectionModel {
            name: "tiny-collection".to_string(),
            size: Some(3126700),
            status: Status::Ready,
            dimension: Some(3),
            vector_count: Some(99),
            environment: "us-east1-gcp".to_string(),
        };

        assert_eq!(collection, expected);
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_describe_collection_invalid_name(
    ) -> Result<(), PineconeError<DescribeCollectionError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/collections/invalid-collection");
            then.status(404)
                .header("content-type", "application/json")
                .body(
                    r#"{
                    "error": "Collection invalid-collection not found"
                }"#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let response = pinecone
            .describe_collection("invalid-collection")
            .await
            .expect_err("Expected describe_collection to return an error");

        assert!(matches!(
            response,
            PineconeError::CollectionNotFoundError { .. }
        ));
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_describe_collection_server_error(
    ) -> Result<(), PineconeError<DescribeCollectionError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/collections/collection-name");
            then.status(500);
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let response = pinecone
            .describe_collection("collection-name")
            .await
            .expect_err("Expected describe_collection to return an error");

        assert!(matches!(
            response,
            PineconeError::InternalServerError { .. }
        ));
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_list_collections() -> Result<(), PineconeError<ListCollectionsError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/collections");
            then.status(200)
                .header("content-type", "application/json")
                .body(
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
                    }"#,
                );
        });

        // Construct Pinecone instance with the mock server URL
        let pinecone = PineconeClient::new(
            Some("api-key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
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

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_list_collections_error() -> Result<(), PineconeError<ListCollectionsError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/collections");
            then.status(500);
        });

        let pinecone = PineconeClient::new(
            Some("api-key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        // Call list_collections and verify the result
        let list_collections_response = pinecone
            .list_collections()
            .await
            .expect_err("Expected to fail to list collections");

        assert!(matches!(
            list_collections_response,
            PineconeError::InternalServerError { .. }
        ));
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_collection() -> Result<(), PineconeError<DeleteCollectionError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(DELETE).path("/collections/collection-name");
            then.status(202);
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let _ = pinecone
            .delete_collection("collection-name")
            .await
            .expect("Failed to delete collection");

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_collection_not_found() -> Result<(), PineconeError<DeleteCollectionError>>
    {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(DELETE).path("/collections/collection-name");
            then.status(404)
                .header("content-type", "application/json")
                .body(
                    r#"
                    {
                        "error": "Collection not found"
                    }
                "#,
                );
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let delete_collection_response = pinecone
            .delete_collection("collection-name")
            .await
            .expect_err("Expected delete_collection to return an error");

        assert!(matches!(
            delete_collection_response,
            PineconeError::CollectionNotFoundError { .. }
        ));

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_collection_internal_error(
    ) -> Result<(), PineconeError<DeleteCollectionError>> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(DELETE).path("/collections/collection-name");
            then.status(500);
        });

        let pinecone = PineconeClient::new(
            Some("api_key".to_string()),
            Some(server.base_url()),
            None,
            None,
        )
        .expect("Failed to create Pinecone instance");

        let delete_collection_response = pinecone
            .delete_collection("collection-name")
            .await
            .expect_err("Expected delete_collection to return an error");

        assert!(matches!(
            delete_collection_response,
            PineconeError::InternalServerError { .. }
        ));

        mock.assert();

        Ok(())
    }
}
