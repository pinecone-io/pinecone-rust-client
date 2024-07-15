use crate::pinecone::PineconeClient;
use crate::utils::errors::PineconeError;
use once_cell::sync::Lazy;
use pb::vector_service_client::VectorServiceClient;
use tonic::metadata::{Ascii, MetadataValue as TonicMetadataVal};
use tonic::service::interceptor::InterceptedService;
use tonic::service::Interceptor;
use tonic::transport::Channel;
use tonic::{Request, Status};

pub use pb::{UpsertResponse, Vector};

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
#[derive(Debug)]
pub struct Index {
    /// The name of the index.
    host: String,
    connection: VectorServiceClient<InterceptedService<Channel, ApiKeyInterceptor>>,
}

impl Index {
    /// The upsert operation writes vectors into a namespace.
    /// If a new value is upserted for an existing vector id, it will overwrite the previous value.
    ///
    /// ### Arguments
    /// * `vectors: Vec<pb::Vector>` - A list of vectors to upsert.
    ///
    /// ### Return
    /// * `Result<UpsertResponse, PineconeError>` - A response object.
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::pinecone::data::pb::Vector;
    /// # use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// let mut index = pinecone.index("index-host").await.unwrap();
    ///
    /// let vectors = vec![Vector {
    ///     id: "vector-id".to_string(),
    ///     values: vec![1.0, 2.0, 3.0, 4.0],
    ///     sparse_values: None,
    ///     metadata: None,
    /// }];
    /// let response = index.upsert(vectors, None).await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upsert(
        &mut self,
        vectors: Vec<pb::Vector>,
        namespace: Option<String>,
    ) -> Result<UpsertResponse, PineconeError> {
        let request = pb::UpsertRequest {
            vectors,
            namespace: namespace.unwrap_or_default(),
        };

        let response = self
            .connection
            .upsert(request)
            .await
            .map_err(|e| PineconeError::UpsertError { inner: Box::new(e) })?
            .into_inner();

        Ok(response)
    }
}

impl PineconeClient {
    /// Match the scheme in a host string.
    ///
    /// ### Arguments
    /// * `host: &str` - The host string to match.
    ///
    /// ### Return
    /// * `bool` - True if the host string contains a scheme, false otherwise.
    fn has_scheme(host: &str) -> bool {
        static RE: Lazy<regex::Regex> = Lazy::new(|| regex::Regex::new(r"^[a-zA-Z]+://").unwrap());
        RE.is_match(host)
    }

    /// Match the port in a host string.
    ///
    /// ### Arguments
    /// * `host: &str` - The host string to match.
    ///
    /// ### Return
    /// * `bool` - True if the host string contains a port, false otherwise.
    fn has_port(host: &str) -> bool {
        static RE: Lazy<regex::Regex> = Lazy::new(|| regex::Regex::new(r":\d+$").unwrap());
        RE.is_match(host)
    }

    /// Target an index for data operations.
    ///
    /// ### Arguments
    /// * `host: &str` - The host of the index to target. If the host does not contain a scheme, it will default to `https://`. If the host does not contain a port, it will default to `443`.
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
    /// let index = pinecone.index("index-host").await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn index(&self, host: &str) -> Result<Index, PineconeError> {
        let endpoint = host.to_string();

        let endpoint = if PineconeClient::has_scheme(&endpoint) {
            endpoint
        } else {
            format!("https://{}", endpoint)
        };

        let endpoint = if PineconeClient::has_port(&endpoint) {
            endpoint
        } else {
            format!("{}:443", endpoint)
        };

        let index = Index {
            host: endpoint.clone(),
            connection: self.new_index_connection(endpoint).await?,
        };

        Ok(index)
    }

    async fn new_index_connection(
        &self,
        host: String,
    ) -> Result<VectorServiceClient<InterceptedService<Channel, ApiKeyInterceptor>>, PineconeError>
    {
        let tls_config = tonic::transport::ClientTlsConfig::default();

        // connect to server
        let endpoint = Channel::from_shared(host)
            .map_err(|e| PineconeError::ConnectionError { inner: Box::new(e) })?
            .tls_config(tls_config)
            .map_err(|e| PineconeError::ConnectionError { inner: Box::new(e) })?;

        let channel = endpoint
            .connect()
            .await
            .map_err(|e| PineconeError::ConnectionError { inner: Box::new(e) })?;

        // add api key in metadata through interceptor
        let token: TonicMetadataVal<_> = self.api_key.parse().unwrap();
        let add_api_key_interceptor = ApiKeyInterceptor { api_token: token };
        let inner = VectorServiceClient::with_interceptor(channel, add_api_key_interceptor);

        Ok(inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[tokio::test]
    async fn test_index_full_endpoint() {
        let server = MockServer::start();

        // server url contains scheme and port
        let _mock = server.mock(|_when, then| {
            then.status(200);
        });

        let pinecone = PineconeClient::new(None, None, None, None).unwrap();

        let index = pinecone.index(server.base_url().as_str()).await.unwrap();

        assert_eq!(index.host, server.base_url());
    }

    #[tokio::test]
    async fn test_index_no_scheme() {
        let server = MockServer::start();

        // server url contains no scheme
        let _mock = server.mock(|_when, then| {
            then.status(200);
        });

        let pinecone = PineconeClient::new(None, None, None, None).unwrap();

        let addr = server.address().to_string();

        let _index = pinecone
            .index(addr.as_str())
            .await
            .expect_err("Expected connection error");
    }

    #[tokio::test]
    async fn test_index_no_port() {
        let server = MockServer::start();

        // server url contains no port
        let _mock = server.mock(|_when, then| {
            then.status(200);
        });

        let pinecone = PineconeClient::new(None, None, None, None).unwrap();

        let scheme_host = format!("http://{}", server.host());

        let _index = pinecone
            .index(scheme_host.as_str())
            .await
            .expect_err("Expected connection error");
    }

    #[tokio::test]
    async fn test_index_no_scheme_no_port() {
        let server = MockServer::start();

        // server url contains no scheme and no port
        let _mock = server.mock(|_when, then| {
            then.status(200);
        });

        let pinecone = PineconeClient::new(None, None, None, None).unwrap();

        let host = server.host();

        let _index = pinecone
            .index(host.as_str())
            .await
            .expect_err("Expected connection error");
    }
}
