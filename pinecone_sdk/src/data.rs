use crate::data::pb::vector_service_client::VectorServiceClient;
use crate::pinecone::PineconeClient;
use crate::utils::errors::PineconeError;
use tonic::metadata::{Ascii, MetadataValue as TonicMetadataVal};
use tonic::service::interceptor::InterceptedService;
use tonic::service::Interceptor;
use tonic::transport::Channel;
use tonic::{Request, Status};

/// Generated protobuf module for data plane.
pub mod pb {
    tonic::include_proto!("_");
}

#[derive(Debug, Clone)]
struct ApiKeyInterceptor {
    api_token: TonicMetadataVal<Ascii>,
}

impl Interceptor for ApiKeyInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        // TODO: replace `api_token` with an `Option`, and do a proper `if_some`.
        if !self.api_token.is_empty() {
            request
                .metadata_mut()
                .insert("api-key", self.api_token.clone());
        }
        Ok(request)
    }
}

/// A client for interacting with a Pinecone index.
pub struct Index {
    /// The name of the index.
    name: String,
    connection: VectorServiceClient<InterceptedService<Channel, ApiKeyInterceptor>>,
}

impl Index {
    /// Upsert a vector
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
    /// # use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// let index = pinecone.index("my-index").await.unwrap();
    /// # Ok(())
    /// # }
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
        // prepend with "https://" if not already present
        let index_host = if index_host.starts_with("https://") {
            index_host
        } else {
            format!("https://{}:443", index_host)
        };
        Ok(index_host)
    }

    async fn new_index_connection(
        &self,
        name: &str,
    ) -> Result<VectorServiceClient<InterceptedService<Channel, ApiKeyInterceptor>>, PineconeError>
    {
        let index_host = self.get_index_host(name).await?;
        let tls_config = tonic::transport::ClientTlsConfig::default();

        // connect to server
        let endpoint = match Channel::from_shared(index_host) {
            Ok(endpoint) => match endpoint.tls_config(tls_config) {
                Ok(channel) => channel,
                Err(e) => {
                    return Err(PineconeError::ConnectionError { inner: Box::new(e) });
                }
            },
            Err(e) => {
                return Err(PineconeError::ConnectionError { inner: Box::new(e) });
            }
        };

        let channel = match endpoint.connect().await {
            Ok(channel) => channel,
            Err(e) => {
                return Err(PineconeError::ConnectionError { inner: Box::new(e) });
            }
        };

        // add api key in metadata through interceptor
        let api_key = self.get_api_key();
        let token: TonicMetadataVal<_> = api_key.parse().unwrap();
        let add_api_key_interceptor = ApiKeyInterceptor { api_token: token };
        let inner = VectorServiceClient::with_interceptor(channel, add_api_key_interceptor);

        Ok(inner)
    }
}
