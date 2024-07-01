use crate::data::pb::vector_service_client::VectorServiceClient;
use crate::pinecone::PineconeClient;
use crate::utils::errors::PineconeError;
use tonic::transport::Channel;

/// Generated protobuf module for data plane.
pub mod pb {
    tonic::include_proto!("_");
}

/// A client for interacting with a Pinecone index.
pub struct Index {
    /// The name of the index.
    name: String,
    connection: VectorServiceClient<Channel>,
}

impl Index {
    pub async fn upsert(&mut self, vectors: Vec<pb::Vector>) -> Result<(), PineconeError> {
        let request = pb::UpsertRequest {
            vectors,
            namespace: "".to_string(),
        };

        let response = match self.connection.upsert(request).await {
            Ok(response) => response,
            Err(e) => {
                return Err(PineconeError::ConnectionError { inner: Box::new(e) });
            }
        };

        Ok(())
    }
}

impl PineconeClient {
    /// Target an index for data operations.
    ///
    /// ### Arguments
    /// * `name: &str` - The name of the index to target.
    ///
    /// ### Return
    /// * `Result<Index, PineconeError>` - A Pinecone index object.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    ///
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    /// let index = pinecone.index("my-index").unwrap();
    /// ```
    pub async fn index(&self, name: &str) -> Result<Index, PineconeError> {
        let index = Index {
            name: name.to_string(),
            connection: self.new_index_connection(name).await?,
        };

        Ok(index)
    }

    async fn get_index_host(&self, name: &str) -> Result<String, PineconeError> {
        let index_host = self.describe_index(name).await?.host;
        // prepend with "http://" if not already present
        let index_host = if index_host.starts_with("http://") {
            index_host
        } else {
            format!("http://{}", index_host)
        };
        Ok(index_host)
    }

    pub async fn new_index_connection(
        &self,
        name: &str,
    ) -> Result<VectorServiceClient<Channel>, PineconeError> {
        let index_host = self.get_index_host(name).await?;
        let connection = match VectorServiceClient::connect(index_host).await {
            Ok(connection) => connection,
            Err(e) => {
                return Err(PineconeError::ConnectionError { inner: Box::new(e) });
            }
        };

        Ok(connection)
    }
}
