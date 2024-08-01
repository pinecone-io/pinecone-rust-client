use std::cmp::min;
use std::time::Duration;

use crate::openapi::apis::manage_indexes_api;
use crate::openapi::models::CreateIndexRequest;
use crate::pinecone::PineconeClient;
use crate::utils::errors::PineconeError;

use crate::models::{
    Cloud, CollectionList, CollectionModel, ConfigureIndexRequest, ConfigureIndexRequestSpec,
    ConfigureIndexRequestSpecPod, CreateCollectionRequest, DeletionProtection, IndexList,
    IndexModel, IndexSpec, Metric, PodSpec, PodSpecMetadataConfig, ServerlessSpec, WaitPolicy,
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
    /// * `deletion_protection: DeletionProtection` - Deletion protection for the index.
    /// * `timeout: WaitPolicy` - The wait policy for index creation. If the index becomes ready before the specified duration, the function will return early. If the index is not ready after the specified duration, the function will return an error.
    ///
    /// ### Return
    /// * `Result<IndexModel, PineconeError>`
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::models::{IndexModel, Metric, Cloud, WaitPolicy, DeletionProtection};
    /// use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(Default::default())?;
    ///
    /// // Create an index.
    /// let response: Result<IndexModel, PineconeError> = pinecone.create_serverless_index(
    ///     "index-name", // Name of the index
    ///     10, // Dimension of the vectors
    ///     Metric::Cosine, // Distance metric
    ///     Cloud::Aws, // Cloud provider
    ///     "us-east-1", // Region
    ///     DeletionProtection::Enabled, // Deletion protection
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
        deletion_protection: DeletionProtection,
        timeout: WaitPolicy,
    ) -> Result<IndexModel, PineconeError> {
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
            deletion_protection: Some(deletion_protection),
            metric: Some(metric.into()),
            spec: Some(Box::new(create_index_request_spec)),
        };

        // make openAPI call
        let res = manage_indexes_api::create_index(&self.openapi_config, create_index_request)
            .await
            .map_err(|e| PineconeError::from(e))?;

        // poll index status
        match self.handle_poll_index(name, timeout).await {
            Ok(_) => Ok(res.into()),
            Err(e) => Err(e),
        }
    }

    /// Creates a pod index.
    ///
    /// ### Arguments
    /// * `name: &str` - The name of the index
    /// * `dimension: i32` - The dimension of the index
    /// * `metric: Metric` - The metric to use for the index
    /// * `environment: &str` - The environment where the pod index will be deployed. Example: 'us-east1-gcp'
    /// * `pod_type: &str` - This value combines pod type and pod size into a single string. This configuration is your main lever for vertical scaling.
    /// * `pods: i32` - The number of pods to deploy.
    /// * `replicas: i32` - The number of replicas to deploy for the pod index.
    /// * `shards: i32` - The number of shards to use. Shards are used to expand the amount of vectors you can store beyond the capacity of a single pod.
    /// * `deletion_protection: DeletionProtection` - Deletion protection for the index.
    /// * `metadata_indexed: Option<&[&str]>` - The metadata fields to index.
    /// * `source_collection: Option<&str>` - The name of the collection to use as the source for the pod index. This configuration is only used when creating a pod index from an existing collection.
    /// * `timeout: WaitPolicy` - The wait policy for index creation. If the index becomes ready before the specified duration, the function will return early. If the index is not ready after the specified duration, the function will return an error.
    ///
    /// ### Return
    /// * Result<IndexModel, PineconeError>`
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::models::{IndexModel, Metric, Cloud, WaitPolicy, DeletionProtection};
    /// use pinecone_sdk::utils::errors::PineconeError;
    /// use std::time::Duration;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError> {
    /// let pinecone = PineconeClient::new(Default::default())?;
    ///
    /// // Create a pod index.
    /// let response: Result<IndexModel, PineconeError> = pinecone.create_pod_index(
    ///     "index_name", // Name of the index
    ///     10, // Dimension of the index
    ///     Metric::Cosine, // Distance metric
    ///     "us-east-1", // Environment
    ///     "p1.x1", // Pod type
    ///     1, // Number of pods
    ///     1, // Number of replicas
    ///     1, // Number of shards
    ///     DeletionProtection::Enabled, // Deletion protection
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
        replicas: i32,
        shards: i32,
        deletion_protection: DeletionProtection,
        metadata_indexed: Option<&[&str]>,
        source_collection: Option<&str>,
        timeout: WaitPolicy,
    ) -> Result<IndexModel, PineconeError> {
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
            deletion_protection: Some(deletion_protection),
            metric: Some(metric.into()),
            spec: Some(Box::new(spec)),
        };

        // make openAPI call
        let res = manage_indexes_api::create_index(&self.openapi_config, create_index_request)
            .await
            .map_err(|e| PineconeError::from(e))?;

        // poll index status
        match self.handle_poll_index(name, timeout).await {
            Ok(_) => Ok(res.into()),
            Err(e) => Err(e),
        }
    }

    // Checks if the index is ready by polling the index status
    async fn handle_poll_index(
        &self,
        name: &str,
        timeout: WaitPolicy,
    ) -> Result<(), PineconeError> {
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
        let res = manage_indexes_api::describe_index(&self.openapi_config, name).await;
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
    /// * `Result<IndexModel, PineconeError>`
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::models::IndexModel;
    /// use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(Default::default())?;
    ///
    /// // Describe an index in the project.
    /// let response: Result<IndexModel, PineconeError> = pinecone.describe_index("index-name").await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn describe_index(&self, name: &str) -> Result<IndexModel, PineconeError> {
        // make openAPI call
        let res = manage_indexes_api::describe_index(&self.openapi_config, name)
            .await
            .map_err(|e| PineconeError::from(e))?;

        Ok(res.into())
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
    /// use pinecone_sdk::models::IndexList;
    /// use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(Default::default())?;
    ///
    /// // List all indexes in the project.
    /// let response: Result<IndexList, PineconeError> = pinecone.list_indexes().await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_indexes(&self) -> Result<IndexList, PineconeError> {
        // make openAPI call
        let res = manage_indexes_api::list_indexes(&self.openapi_config)
            .await
            .map_err(|e| PineconeError::from(e))?;

        Ok(res.into())
    }

    /// Configures an index.
    ///
    /// This operation changes the deletion protection specification, the pod type, and the number of replicas for an index.
    /// Deletion protection can be changed for both pod and serverless indexes, while pod types and number of replicas can only be changed for pod indexes.
    ///
    /// ### Arguments
    /// * name: &str - The name of the index to be configured.
    /// * deletion_protection: Option<DeletionProtection> - Deletion protection for the index.
    /// * replicas: Option<i32> - The desired number of replicas, lowest value is 0. This parameter should be `None` if the index is serverless.
    /// * pod_type: Option<&str> - The new pod_type for the index. This parameter should be `None` if the index is serverless.
    ///
    /// ### Return
    /// * `Result<IndexModel, PineconeError>`
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::models::{DeletionProtection, IndexModel};
    /// use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(Default::default())?;
    ///
    /// // Configure an index in the project.
    /// let response: Result<IndexModel, PineconeError> = pinecone.configure_index(
    ///     "index-name",
    ///     Some(DeletionProtection::Enabled),
    ///     Some(6),
    ///     Some("s1.x1")
    /// ).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn configure_index(
        &self,
        name: &str,
        deletion_protection: Option<DeletionProtection>,
        replicas: Option<i32>,
        pod_type: Option<&str>,
    ) -> Result<IndexModel, PineconeError> {
        if replicas == None && pod_type == None && deletion_protection == None {
            return Err(PineconeError::InvalidConfigurationError {
                message: "At least one of deletion_protection, number of replicas, or pod type must be provided".to_string(),
            });
        }

        let spec = match (replicas, pod_type) {
            (Some(replicas), Some(pod_type)) => Some(Box::new(ConfigureIndexRequestSpec {
                pod: Box::new(ConfigureIndexRequestSpecPod {
                    replicas: Some(replicas),
                    pod_type: Some(pod_type.to_string()),
                }),
            })),
            (Some(replicas), None) => Some(Box::new(ConfigureIndexRequestSpec {
                pod: Box::new(ConfigureIndexRequestSpecPod {
                    replicas: Some(replicas),
                    pod_type: None,
                }),
            })),
            (None, Some(pod_type)) => Some(Box::new(ConfigureIndexRequestSpec {
                pod: Box::new(ConfigureIndexRequestSpecPod {
                    replicas: None,
                    pod_type: Some(pod_type.to_string()),
                }),
            })),
            (None, None) => None,
        };

        let configure_index_request = ConfigureIndexRequest {
            spec,
            deletion_protection,
        };

        // make openAPI call
        let res = manage_indexes_api::configure_index(
            &self.openapi_config,
            name,
            configure_index_request,
        )
        .await
        .map_err(|e| PineconeError::from(e))?;

        Ok(res.into())
    }

    /// Deletes an index.
    ///
    /// ### Arguments
    /// * name: &str - The name of the index to be deleted.
    ///
    /// ### Return
    /// * `Result<(), PineconeError>`
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(Default::default())?;
    ///
    /// // Delete an index in the project.
    /// let response: Result<(), PineconeError> = pinecone.delete_index("index-name").await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_index(&self, name: &str) -> Result<(), PineconeError> {
        // make openAPI call
        let res = manage_indexes_api::delete_index(&self.openapi_config, name)
            .await
            .map_err(|e| PineconeError::from(e))?;

        Ok(res)
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
    /// use pinecone_sdk::models::CollectionModel;
    /// use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(Default::default())?;
    ///
    /// // Describe an index in the project.
    /// let response: Result<CollectionModel, PineconeError> = pinecone.create_collection("collection-name", "index-name").await;
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

        // make openAPI call
        let res =
            manage_indexes_api::create_collection(&self.openapi_config, create_collection_request)
                .await
                .map_err(|e| PineconeError::from(e))?;

        Ok(res)
    }

    /// Describe a collection.
    ///
    /// ### Arguments
    /// * name: &str - The name of the collection to describe.
    ///
    /// ### Return
    /// * `Result<(), PineconeError>`
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::models::CollectionModel;
    /// use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(Default::default())?;
    ///
    /// // Describe a collection in the project.
    /// let collection: CollectionModel = pinecone.describe_collection("collection-name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn describe_collection(&self, name: &str) -> Result<CollectionModel, PineconeError> {
        let res = manage_indexes_api::describe_collection(&self.openapi_config, name)
            .await
            .map_err(|e| PineconeError::from(e))?;

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
    /// use pinecone_sdk::models::CollectionList;
    /// use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(Default::default())?;
    ///
    /// // List all collections in the project.
    /// let response: Result<CollectionList, PineconeError> = pinecone.list_collections().await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_collections(&self) -> Result<CollectionList, PineconeError> {
        // make openAPI call
        let res = manage_indexes_api::list_collections(&self.openapi_config)
            .await
            .map_err(|e| PineconeError::from(e))?;

        Ok(res)
    }

    /// Deletes a collection.
    ///
    /// ### Arguments
    /// * name: &str - The name of the collection to be deleted.
    ///
    /// ### Return
    /// * `Result<(), PineconeError>`
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(Default::default())?;
    ///
    /// // Delete a collection in the project.
    /// let response: Result<(), PineconeError> = pinecone.delete_collection("collection-name").await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_collection(&self, name: &str) -> Result<(), PineconeError> {
        // make openAPI call
        let res = manage_indexes_api::delete_collection(&self.openapi_config, name)
            .await
            .map_err(|e| PineconeError::from(e))?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openapi::{
        self,
        models::{self, collection_model::Status},
    };
    use crate::pinecone::PineconeClientConfig;
    use httpmock::prelude::*;
    use tokio;

    #[tokio::test]
    async fn test_create_serverless_index() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_serverless_index(
                "index-name",
                10,
                Metric::Cosine,
                Cloud::Aws,
                "us-east-1",
                DeletionProtection::Enabled,
                WaitPolicy::NoWait,
            )
            .await
            .expect("Failed to create serverless index");

        mock.assert();

        assert_eq!(create_index_response.name, "index-name");
        assert_eq!(create_index_response.dimension, 10);
        assert_eq!(create_index_response.metric, Metric::Euclidean);

        let spec = create_index_response.spec.serverless.unwrap();
        assert_eq!(spec.cloud, openapi::models::serverless_spec::Cloud::Aws);
        assert_eq!(spec.region, "us-east-1");

        Ok(())
    }

    #[tokio::test]
    async fn test_create_serverless_index_defaults() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_serverless_index(
                "index-name",
                10,
                Default::default(),
                Default::default(),
                "us-east-1",
                DeletionProtection::Enabled,
                WaitPolicy::NoWait,
            )
            .await
            .expect("Failed to create serverless index");

        assert_eq!(create_index_response.name, "index-name");
        assert_eq!(create_index_response.dimension, 10);
        assert_eq!(create_index_response.metric, Metric::Cosine);

        let spec = create_index_response.spec.serverless.unwrap();
        assert_eq!(spec.cloud, openapi::models::serverless_spec::Cloud::Gcp);
        assert_eq!(spec.region, "us-east-1");

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_create_serverless_index_invalid_region() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_serverless_index(
                "index-name",
                10,
                Default::default(),
                Default::default(),
                "abc",
                DeletionProtection::Enabled,
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
    async fn test_create_serverless_index_index_exists() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_serverless_index(
                "index-name",
                10,
                Default::default(),
                Default::default(),
                "us-west-1",
                DeletionProtection::Enabled,
                WaitPolicy::NoWait,
            )
            .await
            .expect_err("Expected error when creating serverless index");

        assert!(matches!(
            create_index_response,
            PineconeError::ResourceAlreadyExistsError { .. }
        ));
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_create_serverless_index_unprocessable_entity() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_serverless_index(
                "index-name",
                10,
                Default::default(),
                Default::default(),
                "us-west-1",
                DeletionProtection::Enabled,
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
    async fn test_create_serverless_index_internal_error() -> Result<(), PineconeError> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/indexes");
            then.status(500);
        });

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_serverless_index(
                "index-name",
                10,
                Metric::Cosine,
                Cloud::Aws,
                "us-east-1",
                DeletionProtection::Enabled,
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
    async fn test_describe_serverless_index() -> Result<(), PineconeError> {
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
                        "deletion_protection": "disabled",
                        "status": {
                            "ready": true,
                            "state": "Ready"
                        }
                    }"#,
                );
        });

        // Construct Pinecone instance with the mock server URL
        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        // Call describe_index and verify the result
        let index = pinecone
            .describe_index("serverless-index")
            .await
            .expect("Failed to describe index");

        let expected = IndexModel {
            name: "serverless-index".to_string(),
            metric: Metric::Cosine,
            dimension: 1536,
            status: openapi::models::IndexModelStatus {
                ready: true,
                state: openapi::models::index_model_status::State::Ready,
            },
            host: "mock-host".to_string(),
            deletion_protection: Some(DeletionProtection::Disabled),
            spec: models::IndexModelSpec {
                serverless: Some(Box::new(models::ServerlessSpec {
                    cloud: openapi::models::serverless_spec::Cloud::Aws,
                    region: "us-east-1".to_string(),
                })),
                pod: None,
            },
        };

        assert_eq!(index, expected);
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_describe_index_invalid_name() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

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
    async fn test_describe_index_server_error() -> Result<(), PineconeError> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/indexes/index-name");
            then.status(500);
        });

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

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
    async fn test_list_indexes() -> Result<(), PineconeError> {
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
        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        // Call list_indexes and verify the result
        let index_list = pinecone
            .list_indexes()
            .await
            .expect("Failed to list indexes");

        let expected = IndexList {
            // name: String, dimension: i32, metric: Metric, host: String, spec: models::IndexModelSpec, status: models::IndexModelStatus)
            indexes: Some(vec![
                IndexModel {
                    name: "index1".to_string(),
                    dimension: 1536,
                    metric: Metric::Cosine,
                    host: "host1".to_string(),
                    deletion_protection: None,
                    spec: models::IndexModelSpec::default(),
                    status: models::IndexModelStatus::default(),
                },
                IndexModel {
                    name: "index2".to_string(),
                    dimension: 1536,
                    metric: Metric::Cosine,
                    host: "host2".to_string(),
                    deletion_protection: None,
                    spec: models::IndexModelSpec::default(),
                    status: models::IndexModelStatus::default(),
                },
            ]),
        };
        assert_eq!(index_list, expected);
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_list_indexes_server_error() -> Result<(), PineconeError> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/indexes");
            then.status(500);
        });

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

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
    async fn test_create_pod_index() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_pod_index(
                "index-name",
                1536,
                Metric::Euclidean,
                "us-east-1-aws",
                "p1.x1",
                1,
                1,
                1,
                DeletionProtection::Enabled,
                Some(&vec!["genre", "title", "imdb_rating"]),
                Some("example-collection"),
                WaitPolicy::NoWait,
            )
            .await
            .expect("Failed to create pod index");

        assert_eq!(create_index_response.name, "index-name");
        assert_eq!(create_index_response.dimension, 1536);
        assert_eq!(create_index_response.metric, Metric::Euclidean);

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
        assert_eq!(pod_spec.replicas, 1);
        assert_eq!(pod_spec.shards, 1);

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_create_pod_index_with_defaults() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_pod_index(
                "index-name",
                1536,
                Default::default(),
                "us-east-1-aws",
                "p1.x1",
                1,
                1,
                1,
                DeletionProtection::Enabled,
                None,
                None,
                WaitPolicy::NoWait,
            )
            .await
            .expect("Failed to create pod index");

        assert_eq!(create_index_response.name, "index-name");
        assert_eq!(create_index_response.dimension, 1536);
        assert_eq!(create_index_response.metric, Metric::Cosine);

        let pod_spec = create_index_response.spec.pod.as_ref().unwrap();
        assert_eq!(pod_spec.environment, "us-east-1-aws");
        assert_eq!(pod_spec.pod_type, "p1.x1");
        assert_eq!(pod_spec.metadata_config.as_ref().unwrap().indexed, None);
        assert_eq!(pod_spec.pods, 1);
        assert_eq!(pod_spec.replicas, 1);
        assert_eq!(pod_spec.shards, 1);

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_create_pod_index_quota_exceeded() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_pod_index(
                "index-name",
                1536,
                Metric::Euclidean,
                "test-environment",
                "p1.x1",
                1,
                1,
                1,
                DeletionProtection::Enabled,
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
    async fn test_create_pod_index_invalid_environment() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_pod_index(
                "index-name",
                1536,
                Metric::Euclidean,
                "invalid-environment",
                "p1.x1",
                1,
                1,
                1,
                DeletionProtection::Enabled,
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
    async fn test_create_pod_index_invalid_pod_type() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let create_index_response = pinecone
            .create_pod_index(
                "index-name",
                1536,
                Metric::Euclidean,
                "us-east-1-aws",
                "invalid-pod-type",
                1,
                1,
                1,
                DeletionProtection::Enabled,
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
    async fn test_handle_polling_index_ok() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let res = pinecone
            .handle_poll_index("index-name", WaitPolicy::WaitFor(Duration::from_secs(1)))
            .await;

        assert!(res.is_ok());
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_handle_polling_index_err() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

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
    async fn test_configure_index() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let configure_index_response = pinecone
            .configure_index(
                "index-name",
                Some(DeletionProtection::Disabled),
                Some(6),
                Some("p1.x1"),
            )
            .await
            .expect("Failed to configure index");

        assert_eq!(configure_index_response.name, "index-name");

        let spec = configure_index_response.spec.pod.unwrap();
        assert_eq!(spec.replicas, 6);
        assert_eq!(spec.pod_type.as_str(), "p1.x1");

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_configure_deletion_protection() -> Result<(), PineconeError> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.path("/indexes/index-name");
            then.status(202)
                .header("content-type", "application/json")
                .body(
                    r#"{
                        "name": "index-name",
                        "dimension": 1536,
                        "metric": "cosine",
                        "host": "mock-host",
                        "deletion_protection": "disabled",
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
                    }"#,
                );
        });

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let configure_index_response = pinecone
            .configure_index("index-name", Some(DeletionProtection::Disabled), None, None)
            .await
            .expect("Failed to configure index");

        assert_eq!(
            configure_index_response.deletion_protection,
            Some(DeletionProtection::Disabled)
        );

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_configure_index_no_params() -> Result<(), PineconeError> {
        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let configure_index_response = pinecone
            .configure_index("index-name", None, None, None)
            .await;

        assert!(matches!(
            configure_index_response,
            Err(PineconeError::InvalidConfigurationError { .. })
        ));

        Ok(())
    }

    #[tokio::test]
    async fn test_configure_index_quota_exceeded() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let configure_index_response = pinecone
            .configure_index(
                "index-name",
                Some(DeletionProtection::Enabled),
                Some(6),
                Some("p1.x1"),
            )
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
    async fn test_configure_index_not_found() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let configure_index_response = pinecone
            .configure_index(
                "index-name",
                Some(DeletionProtection::Disabled),
                Some(6),
                Some("p1.x1"),
            )
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
    async fn test_configure_index_unprocessable_entity() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let configure_index_response = pinecone
            .configure_index(
                "index-name",
                Some(DeletionProtection::Enabled),
                Some(6),
                Some("p1.x1"),
            )
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
    async fn test_configure_index_internal_error() -> Result<(), PineconeError> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.path("/indexes/index-name");
            then.status(500);
        });

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let configure_index_response = pinecone
            .configure_index(
                "index-name",
                Some(DeletionProtection::Enabled),
                Some(6),
                Some("p1.x1"),
            )
            .await
            .expect_err("Expected to fail to configure index");

        assert!(matches!(
            configure_index_response,
            PineconeError::InternalServerError { .. }
        ));

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_index() -> Result<(), PineconeError> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(DELETE).path("/indexes/index-name");
            then.status(202);
        });

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let _ = pinecone
            .delete_index("index-name")
            .await
            .expect("Failed to delete index");

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_index_invalid_name() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

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
    async fn test_delete_index_pending_collection() -> Result<(), PineconeError> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(DELETE).path("/indexes/index-name");
            then.status(412);
        });

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

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
    async fn test_delete_index_server_error() -> Result<(), PineconeError> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(DELETE).path("/indexes/index-name");
            then.status(500);
        });

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

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
    async fn test_create_collection() -> Result<(), PineconeError> {
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
        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

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
    async fn test_create_collection_quota_exceeded() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

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
    async fn test_create_collection_invalid_name() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let create_collection_response = pinecone
            .create_collection("invalid_collection", "valid-index")
            .await
            .expect_err("Expected create_collection to return an error");

        assert!(matches!(
            create_collection_response,
            PineconeError::ResourceAlreadyExistsError { .. }
        ));

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_create_collection_server_error() -> Result<(), PineconeError> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/collections");
            then.status(500);
        });

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

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
    async fn test_describe_collection() -> Result<(), PineconeError> {
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
        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

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
    async fn test_describe_collection_invalid_name() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

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
    async fn test_describe_collection_server_error() -> Result<(), PineconeError> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/collections/collection-name");
            then.status(500);
        });

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

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
    async fn test_list_collections() -> Result<(), PineconeError> {
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
        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

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
    async fn test_list_collections_error() -> Result<(), PineconeError> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/collections");
            then.status(500);
        });

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

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
    async fn test_delete_collection() -> Result<(), PineconeError> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(DELETE).path("/collections/collection-name");
            then.status(202);
        });

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

        let _ = pinecone
            .delete_collection("collection-name")
            .await
            .expect("Failed to delete collection");

        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_collection_not_found() -> Result<(), PineconeError> {
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

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

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
    async fn test_delete_collection_internal_error() -> Result<(), PineconeError> {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(DELETE).path("/collections/collection-name");
            then.status(500);
        });

        let params = PineconeClientConfig {
            api_key: Some("api_key".to_string()),
            control_plane_host: Some(server.base_url()),
            ..Default::default()
        };
        let pinecone = PineconeClient::new(params).expect("Failed to create Pinecone instance");

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
