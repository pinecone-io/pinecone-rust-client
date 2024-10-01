//! # Pinecone Rust SDK
//!
//! > ⚠️ **Warning:** This SDK is still in an alpha state. While it is mostly built out and functional,
//! > it may undergo changes as we continue to improve the UX. Please try it out and give us your feedback,
//! > but be aware that updates may introduce breaking changes.
//!
//! `pinecone-sdk` provides an interface for working with Pinecone services such as the Database and Inference APIs.
//! See [docs.pinecone.io](https://docs.pinecone.io/) for more information.
//!
//! Before you can interact with Pinecone services, you'll need to create an account and generate an
//! API key. This can be done through the Pinecone console: [https://app.pinecone.io](https://app.pinecone.io).
//!
//! ## Using the SDK
//!
//! The `PineconeClient` struct is the main point of entry into the Rust SDK.
//! Parameters may either be directly passed through `PineconeClientConfig`, or read through environment variables:
//!
//! - (Required) The API key must be provided through `PineconeClientConfig.api_key`, or an environment variable named `PINECONE_API_KEY`.
//!   If passed in as `None`, the client will attempt to read in an environment variable value, otherwise the passed value will be used.
//! - (Optional) The control plane host, if passed in as `None`, will attempt to read in an environment variable called `PINECONE_CONTROLLER_HOST`.
//!   If it is not an environment variable, it will default to `https://api.pinecone.io`.
//!
//! There are two ways of initializing a `PineconeClient`. The only required parameter for working with Pinecone is an API key:
//!
//! Use the `default_client()` function, which is the equivalent of constructing a `PineconeClientConfig` struct with all fields set to `None`.
//! The API key will be read from environment variables
//! ```no_run
//! use pinecone_sdk::pinecone::PineconeClient;
//! let client: PineconeClient = pinecone_sdk::pinecone::default_client().expect("Failed to create Pinecone instance");
//! ```
//!
//! Initialize a `PineconeClientConfig` struct with parameters, and call `client()` to create a `PineconeClient` instance:
//! ```no_run
//! use pinecone_sdk::pinecone::{PineconeClient, PineconeClientConfig};
//!
//! let config = PineconeClientConfig {
//!     api_key: Some("INSERT_API_KEY".to_string()),
//!     ..Default::default()
//! };
//!
//! let client: PineconeClient = config.client().expect("Failed to create Pinecone instance");
//! ```
//! Once you have a `PineconeClient` instance, you can use it to work with Pinecone services.
//!
//! ### Working with Indexes and Collections
//!
//! Indexes and collections can be managed with the `PineconeClient` instance directly.
//!
//! ```no_run
//! use pinecone_sdk::pinecone;
//! use pinecone_sdk::models::{Cloud, DeletionProtection, IndexModel, Metric, WaitPolicy};
//! use pinecone_sdk::utils::errors::PineconeError;
//! # async fn create_index_and_collection() -> Result<(), PineconeError> {
//!     let client: pinecone::PineconeClient =
//!     pinecone::default_client().expect("Failed to create PineconeClient");
//!
//!     let index: IndexModel = client
//!         .create_serverless_index(
//!             "my-index-name",
//!             10,
//!             Metric::Cosine,
//!             Cloud::Aws,
//!             "us-east-1",
//!             DeletionProtection::Disabled,
//!             WaitPolicy::NoWait,
//!         )
//!         .await?;
//!
//!     let collection = client.create_collection("my-collection-name", "my-previous-index-name").await?;
//!
//!     let index_description = client.describe_index("index-name").await?;
//!     let collection_description = client.describe_collection("my-collection-name").await?;
//!     let indexes = client.list_indexes().await?;
//!
//!     println!("Index description: {:?}", index_description);
//!     println!("Collection description: {:?}", collection_description);
//!     println!("Index list: {:?}", indexes);
//!
//! #   Ok(())
//! # }
//! ```
//!
//! ### Connecting to an Index
//!
//! Once you have an index created and you want to work with data, you will want to create an `Index` instance by using
//! the `index()` method on the `PineconeClient` instance. You will need to provide the `host` of the index you are targeting
//! which can be found by using the `describe_index()` or `list_indexes()` methods.
//!
//! ```no_run
//! use pinecone_sdk::pinecone;
//! use pinecone_sdk::models::{QueryResponse, Vector};
//! use pinecone_sdk::utils::errors::PineconeError;
//! # async fn upsert_and_query_vectors() -> Result<(), PineconeError> {
//!     let client = pinecone::default_client().expect("Failed to initialize PineconeClient");
//!     let index_description = client.describe_index("my-index").await?;
//!     let mut index = client.index(&index_description.host).await?;
//!
//!     // upsert vectors
//!     let vectors = [Vector {
//!         id: "id1".to_string(),
//!         values: vec![1.0, 2.0, 3.0, 4.0],
//!         sparse_values: None,
//!         metadata: None,
//!     }, Vector {
//!         id: "id2".to_string(),
//!         values: vec![2.0, 3.0, 4.0, 5.0],
//!         sparse_values: None,
//!         metadata: None,
//!     }];
//!
//!     let upsert_response = index.upsert(&vectors, &"my-namespace".into()).await?;
//!     println!("Upserted {:?} vectors", upsert_response.upserted_count);
//!
//!     // query vectors
//!     let query_vector = vec![1.0, 2.0, 3.0, 4.0];
//!
//!     let query_response: QueryResponse = index
//!         .query_by_value(query_vector, None, 10, &"my-namespace".into(), None, None, None)
//!         .await?;
//!     println!("Query response: {:?}", query_response);
//!
//! #   Ok(())
//! # }
//! ```
//!
//! ### Working with Inference
//!
//! The Inference API is a service that gives you access to embedding models hosted on Pinecone's infrastructure.
//! Read more at [Understanding Pinecone Inference](https://docs.pinecone.io/guides/inference/understanding-inference).
//!
//! ```
//! use pinecone_sdk::pinecone;
//! use pinecone_sdk::models::{EmbedRequestParameters};
//! use pinecone_sdk::utils::errors::PineconeError;
//! # async fn embed() -> Result<(), PineconeError> {
//!     let client = pinecone::default_client().expect("Failed to initialize PineconeClient");
//!     let embeddings = client
//!     .embed(
//!         "multilingual-e5-large",
//!         Some(EmbedRequestParameters {
//!             input_type: Some("passage".to_string()),
//!             truncate: Some("END".to_string()),
//!         }),
//!         &vec![
//!             "Turkey is a classic meat to eat at American Thanksgiving.",
//!             "Many people enjoy the beautiful mosques in Turkey.",
//!         ],
//!     )
//!     .await
//!     .expect("Failed to embed");
//!
//!     println!("Embeddings: {:?}", embeddings);
//! #    Ok(())
//! # }
//! ```
//!
//! For more detailed documentation on Pinecone see [https://docs.pinecone.io](https://docs.pinecone.io).

#![warn(missing_docs)]

/// Defines the main entrypoint of the Pinecone SDK.
pub mod pinecone;

/// Utility modules.
pub mod utils;

/// Models for the Pinecone SDK.
pub mod models;

/// Version information.
pub mod version;

/// OpenAPI client for Pinecone.
#[allow(missing_docs)]
#[allow(dead_code)]
mod openapi;

/// Protobuf client for Pinecone.
#[allow(missing_docs)]
#[allow(dead_code)]
mod protos;
