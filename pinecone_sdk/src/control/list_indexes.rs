use crate::pinecone::PineconeClient;
use openapi::apis::manage_indexes_api;
use openapi::apis::manage_indexes_api::ListIndexesError;
use openapi::apis::Error;
use openapi::models;

impl PineconeClient {
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

    pub async fn list_indexes(&self) -> Result<models::IndexList, Error<ListIndexesError>> {
        let response = manage_indexes_api::list_indexes(self.openapi_config()).await?;
        println!("{:?}", response);
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::list_indexes::models::index_model::Metric;
    use crate::utils::errors::PineconeError;
    use mockito::mock;
    use openapi::models::IndexList;
    use openapi::models::IndexModel;
    use tokio;

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

        Ok(())
    }
}
