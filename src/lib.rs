//! # Pinecone Rust SDK
//!
//! The official Pinecone Rust client.
//!
//! For more information, see the docs at [https://docs.pinecone.io](https://docs.pinecone.io).

#![warn(missing_docs)]

/// Defines the main entrypoint of the Pinecone SDK.
pub mod pinecone;

/// Utility modules.
pub mod utils;

/// OpenAPI client for Pinecone.
#[allow(missing_docs)]
#[allow(dead_code)]
mod openapi;

/// Protobuf client for Pinecone.
#[allow(missing_docs)]
#[allow(dead_code)]
mod protos;

/// Models for the Pinecone SDK.
pub mod models;

/// Version information.
pub mod version;
