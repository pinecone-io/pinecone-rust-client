use crate::pinecone::PineconeClient;
use crate::utils::errors::PineconeError;
use openapi::apis::manage_indexes_api;
use openapi::models::{
    CreateIndexRequest, CreateIndexRequestSpec, IndexList, IndexModel, PodSpec,
    PodSpecMetadataConfig, ServerlessSpec,
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
    ///
    /// ### Return
    /// * Returns a `Result<IndexModel, PineconeError>` object.
    ///
    /// ### Example
    /// ```
    /// // put a good example here of how the developer should use it
    /// # // start the line like this if you don't want this to be shown to the dev
    /// # // but it's needed to run properly during doc tests
    /// ```
    ///
    /// Include any additional technical notes here.
    pub async fn create_pod_index(
        &self,
        name: String,
        dimension: u32,
        metric: Metric,
        environment: String,
        replicas: Option<i32>,
        shards: Option<i32>,
        pod_type: String,
        pods: i32,
        indexed: Option<Vec<String>>,
        source_collection: Option<String>,
    ) -> Result<IndexModel, PineconeError> {
        let pod_spec = PodSpec {
            environment,
            replicas,
            shards,
            pod_type,
            pods,
            metadata_config: Some(Box::new(PodSpecMetadataConfig { indexed })),
            source_collection,
        };

        let spec = CreateIndexRequestSpec {
            serverless: None,
            pod: Some(Box::new(pod_spec)),
        };

        let create_index_request = CreateIndexRequest {
            name,
            dimension: dimension.try_into().unwrap(),
            metric: Some(metric),
            spec: Some(Box::new(spec)),
        };

        match manage_indexes_api::create_index(&self.openapi_config(), create_index_request).await {
            Ok(response) => Ok(response),
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
                "test-index".to_string(),
                1536,
                Metric::Euclidean,
                "us-east-1-aws".to_string(),
                Some(1),
                Some(1),
                "p1.x1".to_string(),
                1,
                Some(vec![
                    "genre".to_string(),
                    "title".to_string(),
                    "imdb_rating".to_string(),
                ]),
                Some("example-collection".to_string()),
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
                "test-index".to_string(),
                1536,
                Default::default(),
                "us-east-1-aws".to_string(),
                None,
                None,
                "p1.x1".to_string(),
                1,
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
        assert_eq!(
            pod_spec.metadata_config.as_ref().unwrap().indexed,
            None
        );
        assert_eq!(pod_spec.pods, 1);
        assert_eq!(pod_spec.replicas, Some(1));
        assert_eq!(pod_spec.shards, Some(1));

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
