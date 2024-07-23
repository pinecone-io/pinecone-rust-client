//! # Pinecone Rust SDK
//!
//! The official Pinecone Rust client.
//!
//! For more information, see the docs at [https://docs.pinecone.io](https://docs.pinecone.io).

#![warn(missing_docs)]

/// Defines configurations for the Pinecone SDK.
pub mod config;

/// Defines the main entrypoint of the Pinecone SDK.
pub mod pinecone;

/// Utility modules.
pub mod utils;

/// OpenAPI client for Pinecone.
pub mod openapi;

/// Protobuf client for Pinecone.
pub mod protos;