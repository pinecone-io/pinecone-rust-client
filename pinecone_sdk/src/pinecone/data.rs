use crate::pinecone::PineconeClient;
use crate::utils::errors::PineconeError;
use once_cell::sync::Lazy;
use pb::vector_service_client::VectorServiceClient;
use tonic::metadata::{Ascii, MetadataValue as TonicMetadataVal};
use tonic::service::interceptor::InterceptedService;
use tonic::service::Interceptor;
use tonic::transport::Channel;
use tonic::{Request, Status};

pub use pb::{
    DescribeIndexStatsResponse, ListResponse, SparseValues, UpdateResponse, UpsertResponse, Vector,
};
pub use prost_types::{value::Kind, Struct as Metadata, Value};

/// Generated protobuf module for data plane.
pub mod pb {
    include!("../../../protos/_.rs");
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

/// The namespace of an index
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct Namespace {
    /// The name of the namespace
    name: String,
}

impl From<String> for Namespace {
    fn from(name: String) -> Self {
        Self { name }
    }
}

impl From<&str> for Namespace {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Index {
    /// The upsert operation writes vectors into a namespace.
    /// If a new value is upserted for an existing vector id, it will overwrite the previous value.
    ///
    /// ### Arguments
    /// * `vectors: Vec<Vector>` - A list of vectors to upsert.
    ///
    /// ### Return
    /// * `Result<UpsertResponse, PineconeError>` - A response object.
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::pinecone::data::{Namespace, Vector};
    /// # use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// let mut index = pinecone.index("index-host").await.unwrap();
    ///
    /// let vectors = [Vector {
    ///     id: "vector-id".to_string(),
    ///     values: vec![1.0, 2.0, 3.0, 4.0],
    ///     sparse_values: None,
    ///     metadata: None,
    /// }];
    /// let response = index.upsert(&vectors, &"namespace".into()).await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upsert(
        &mut self,
        vectors: &[Vector],
        namespace: &Namespace,
    ) -> Result<UpsertResponse, PineconeError> {
        let request = pb::UpsertRequest {
            vectors: vectors.to_vec(),
            namespace: namespace.name.clone(),
        };

        let response = self
            .connection
            .upsert(request)
            .await
            .map_err(|e| PineconeError::DataPlaneError { status: e })?
            .into_inner();

        Ok(response)
    }

    /// The list operation lists the IDs of vectors in a single namespace of a serverless index. An optional prefix can be passed to limit the results to IDs with a common prefix.
    ///
    /// ### Arguments
    /// * `namespace: Namespace` - Default is "".
    /// * `prefix: Option<String>` - The maximum number of vectors to return. If unspecified, the server will use a default value.
    /// * `limit: Option<u32>` - The maximum number of vector ids to return. If unspecified, the default limit is 100.
    /// * `pagination_token: Option<String>` - The token for paginating through results.
    ///
    /// ### Return
    /// * `Result<ListResponse, PineconeError>` - A response object.
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::pinecone::data::Namespace;
    /// # use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// let mut index = pinecone.index("index-host").await.unwrap();
    ///
    /// let response = index.list(&"namespace".into(), None, None, None).await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &mut self,
        namespace: &Namespace,
        prefix: Option<String>,
        limit: Option<u32>,
        pagination_token: Option<String>,
    ) -> Result<ListResponse, PineconeError> {
        let request = pb::ListRequest {
            namespace: namespace.name.clone(),
            prefix,
            limit,
            pagination_token,
        };

        let response = self
            .connection
            .list(request)
            .await
            .map_err(|e| PineconeError::DataPlaneError { status: e })?
            .into_inner();

        Ok(response)
    }

    /// The describe_index_stats operation returns statistics about the index.
    ///
    /// ### Arguments
    /// * `filter: Option<Metadata>` - An optional filter to specify which vectors to return statistics for. Note that the filter is only supported by pod indexes.
    ///
    /// ### Return
    /// * Returns a `Result<DescribeIndexStatsResponse, PineconeError>` object.
    ///
    /// ### Example
    /// ```no_run
    /// use std::collections::BTreeMap;
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::pinecone::data::{Value, Kind, Metadata, Namespace};
    /// # use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// let mut index = pinecone.index("index-host").await.unwrap();
    ///
    /// let mut fields = BTreeMap::new();
    /// fields.insert("field".to_string(), Value { kind: Some(Kind::StringValue("value".to_string())) });
    ///
    /// let response = index.describe_index_stats(Some(Metadata { fields })).await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn describe_index_stats(
        &mut self,
        filter: Option<Metadata>,
    ) -> Result<DescribeIndexStatsResponse, PineconeError> {
        let request = pb::DescribeIndexStatsRequest { filter };

        let response = self
            .connection
            .describe_index_stats(request)
            .await
            .map_err(|e| PineconeError::DataPlaneError { status: e })?
            .into_inner();

        Ok(response)
    }

    /// The update operation updates a vector in a namespace. If a value is included, it will overwrite the previous value.
    /// If a `metadata` filter is included, the values of the fields specified in it will be added or overwrite the previous values.
    ///
    /// ### Arguments
    /// * `id: String` - The vector's unique ID.
    /// * `values: Vec<f32>` - The vector data.
    /// * `sparse_values: Option<SparseValues>` - The sparse vector data.
    /// * `metadata: Option<MetadataFilter>` - The metadata to set for the vector.
    /// * `namespace: Namespace` - The namespace containing the vector to update. Default is "".
    ///
    /// ### Return
    /// * `Result<UpsertResponse, PineconeError>` - A response object.
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::pinecone::data::{Namespace, SparseValues, Metadata};
    /// # use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// let mut index = pinecone.index("index-host").await.unwrap();
    ///
    /// let response = index.update("vector-id".to_string(), vec![1.0, 2.0, 3.0, 4.0], None, None, &"namespace".into()).await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(
        &mut self,
        id: String,
        values: Vec<f32>,
        sparse_values: Option<SparseValues>,
        metadata: Option<Metadata>,
        namespace: &Namespace,
    ) -> Result<UpdateResponse, PineconeError> {
        let request = pb::UpdateRequest {
            id,
            values,
            sparse_values,
            set_metadata: metadata,
            namespace: namespace.name.clone(),
        };

        let response = self
            .connection
            .update(request)
            .await
            .map_err(|e| PineconeError::DataPlaneError { status: e })?
            .into_inner();

        Ok(response)
    }

    /// The delete_by_id operation deletes vectors by ID from a namespace.
    ///
    /// ### Arguments
    /// * `ids: Vec<String>` - List of IDs of vectors to be deleted.
    /// * `namespace: Namespace` - The namespace to delete vectors from. Default is "".
    ///
    /// ### Return
    /// * Returns a `Result<(), PineconeError>` object.
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::pinecone::data::Namespace;
    /// # use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// let mut index = pinecone.index("index-host").await.unwrap();
    ///
    /// let ids = ["vector-id".to_string()];
    /// let response = index.delete_by_id(&ids, &"namespace".into()).await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_by_id(
        &mut self,
        ids: &[String],
        namespace: &Namespace,
    ) -> Result<(), PineconeError> {
        let request = pb::DeleteRequest {
            ids: ids.to_vec(),
            delete_all: false,
            namespace: namespace.name.clone(),
            filter: None,
        };

        self.delete(request).await
    }

    /// The delete_all operation deletes all vectors from a namespace.
    ///
    /// ### Arguments
    /// * `namespace: Namespace` - The namespace to delete vectors from. Default is "".
    ///
    /// ### Return
    /// * Returns a `Result<(), PineconeError>` object.
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::pinecone::data::Namespace;
    /// # use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// let mut index = pinecone.index("index-host").await.unwrap();
    ///
    /// let response = index.delete_all(&"namespace".into()).await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_all(&mut self, namespace: &Namespace) -> Result<(), PineconeError> {
        let request = pb::DeleteRequest {
            ids: vec![],
            delete_all: true,
            namespace: namespace.name.clone(),
            filter: None,
        };

        self.delete(request).await
    }

    /// The delete_by_filter operation deletes the vectors from a namespace that satisfy the filter.
    ///
    /// ### Arguments
    /// * `filter: Metadata` - The filter to specify which vectors to delete.
    /// * `namespace: Namespace` - The namespace to delete vectors from.
    ///
    /// ### Return
    /// * Returns a `Result<(), PineconeError>` object.
    ///
    /// ### Example
    /// ```no_run
    /// use std::collections::BTreeMap;
    /// use pinecone_sdk::pinecone::PineconeClient;
    /// use pinecone_sdk::pinecone::data::{Metadata, Value, Kind, Namespace};
    /// # use pinecone_sdk::utils::errors::PineconeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), PineconeError>{
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    ///
    /// let mut index = pinecone.index("index-host").await.unwrap();
    ///
    /// let mut fields = BTreeMap::new();
    /// fields.insert("field".to_string(), Value { kind: Some(Kind::StringValue("value".to_string())) });
    ///
    /// let response = index.delete_by_filter(Metadata { fields }, &"namespace".into()).await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_by_filter(
        &mut self,
        filter: Metadata,
        namespace: &Namespace,
    ) -> Result<(), PineconeError> {
        let request = pb::DeleteRequest {
            ids: vec![],
            delete_all: false,
            namespace: namespace.name.clone(),
            filter: Some(filter),
        };

        self.delete(request).await
    }

    // Helper function to call delete operation
    async fn delete(&mut self, request: pb::DeleteRequest) -> Result<(), PineconeError> {
        let _ = self
            .connection
            .delete(request)
            .await
            .map_err(|e| PineconeError::DataPlaneError { status: e })?;

        Ok(())
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
            .map_err(|e| PineconeError::ConnectionError {
                source: Box::new(e),
            })?
            .tls_config(tls_config)
            .map_err(|e| PineconeError::ConnectionError {
                source: Box::new(e),
            })?;

        let channel = endpoint
            .connect()
            .await
            .map_err(|e| PineconeError::ConnectionError {
                source: Box::new(e),
            })?;

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
