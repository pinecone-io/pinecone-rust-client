use std::num::NonZero;

use openapi::apis::{
    manage_indexes_api::{
        ConfigureIndexError, CreateCollectionError, CreateIndexError, DeleteCollectionError,
        DeleteIndexError, DescribeIndexError, ListCollectionsError, ListIndexesError,
    },
    Error as OpenAPIError,
};
use snafu::prelude::*;

/// PineconeError is the error type for all Pinecone SDK errors.
#[derive(Debug, Snafu)]
pub enum PineconeError {
    /// APIKeyMissingError: API key is not provided as an argument nor in the environment variable `PINECONE_API_KEY`.
    #[snafu(display("API Key is missing"))]
    APIKeyMissingError,

    /// ConfigureIndexError: Failed to configure an index.
    #[snafu(display("Failed to configure index: {}", msg))]
    ConfigureIndexError {
        status: NonZero<u16>,
        msg: String,
    },

    /// CreateCollectionError: Failed to create a collection.
    #[snafu(display("Failed to create collection: {}", msg))]
    CreateCollectionError {
        status: NonZero<u16>,
        msg: String,
    },

    /// CreateIndexError: Failed to create an index.
    #[snafu(display("Failed to create an index: {}", msg))]
    CreateIndexError {
        status: u64,
        msg: String,
    },

    /// DeleteCollectionError: Failed to delete an index.
    #[snafu(display("Failed to delete collection: {}", msg))]
    DeleteCollectionError {
        status: NonZero<u16>,
        msg: String,
    },

    /// DeleteIndexError: Failed to delete an index.
    #[snafu(display("Failed to delete index: {}", msg))]
    DeleteIndexError {
        status: NonZero<u16>,
        msg: String,
    },

    /// DescribeIndexError: Failed to describe an index.
    #[snafu(display("Failed to describe the index"))]
    DescribeIndexError {
        status: NonZero<u16>,
        msg: String,
    },

    /// InvalidCloudError: Provided cloud is not valid.
    #[snafu(display("Invalid cloud."))]
    InvalidCloudError {
        status: NonZero<u16>,
        msg: String,
    },

    /// InvalidHeadersError: Provided headers are not valid. Expects JSON.
    #[snafu(display("Failed to parse headers: {}", json_error))]
    InvalidHeadersError { json_error: serde_json::Error },

    /// InvalidMetricError: Provided metric is not valid.
    #[snafu(display("Invalid metric."))]
    InvalidMetricError {
        status: NonZero<u16>,
        msg: String,
    },

    /// ListCollectionsError: Failed to list indexes.
    #[snafu(display("Failed to list collections: {}", msg))]
    ListCollectionsError {
        status: NonZero<u16>,
        msg: String,
    },

    /// ListIndexesError: Failed to list indexes.
    #[snafu(display("Failed to list indexes: {}", msg))]
    ListIndexesError {
        status: NonZero<u16>,
        msg: String,
    },

    /// MissingDimensionError: Index dimension is missing.
    #[snafu(display("Dimension missing."))]
    MissingDimensionError {
        status: NonZero<u16>,
        msg: String,
    },

    /// MissingNameError: Index name is missing.
    #[snafu(display("Index name missing."))]
    MissingNameError {
        status: NonZero<u16>,
        msg: String,
    },

    /// MissingSpecError: Index spec is missing.
    #[snafu(display("Spec missing."))]
    MissingSpecError {
        status: NonZero<u16>,
        msg: String,
    },

    /// TimeoutError: Request timed out.
    #[snafu(display("Request timed out."))]
    TimeoutError {
        status: NonZero<u16>,
        msg: String,
    },
}
