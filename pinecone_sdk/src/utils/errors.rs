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
    #[snafu(display("API key missing."))]
    APIKeyMissingError,

    /// ConfigureIndexError: Failed to configure an index.
    #[snafu(display("Failed to configure index '{}'.", name))]
    ConfigureIndexError {
        /// name: Index name.
        name: String,
        /// openapi_error: Error object for OpenAPI error.
        openapi_error: OpenAPIError<ConfigureIndexError>,
    },

    /// ConnectionError: Failed to establish a connection.
    ConnectionError {
        /// inner: Error object for connection error.
        inner: Box<dyn std::error::Error>,
    },

    /// CreateCollectionError: Failed to create a collection.
    #[snafu(display("Failed to create collection '{}'.", name))]
    CreateCollectionError {
        /// name: Collection name.
        name: String,
        /// openapi_error: Error object for OpenAPI error.
        openapi_error: OpenAPIError<CreateCollectionError>,
    },

    /// CreateIndexError: Failed to create an index.
    #[snafu(display("Failed to create an index."))]
    CreateIndexError {
        /// openapi_error: Error object for OpenAPI error.
        openapi_error: OpenAPIError<CreateIndexError>,
    },

    /// DeleteCollectionError: Failed to delete an index.
    #[snafu(display("Failed to delete collection '{}'.", name))]
    DeleteCollectionError {
        /// name: Index name.
        name: String,
        /// openapi_error: Error object for OpenAPI error.
        openapi_error: OpenAPIError<DeleteCollectionError>,
    },

    /// DeleteIndexError: Failed to delete an index.
    #[snafu(display("Failed to delete index '{}'.", name))]
    DeleteIndexError {
        /// name: Index name.
        name: String,
        /// openapi_error: Error object for OpenAPI error.
        openapi_error: OpenAPIError<DeleteIndexError>,
    },

    /// DescribeIndexError: Failed to describe an index.
    #[snafu(display("Failed to describe the index '{}'.", name))]
    DescribeIndexError {
        /// name: Index name.
        name: String,
        /// openapi_error: Error object for OpenAPI error.
        openapi_error: OpenAPIError<DescribeIndexError>,
    },

    /// InvalidCloudError: Provided cloud is not valid.
    #[snafu(display("Invalid cloud '{}'.", cloud))]
    InvalidCloudError {
        /// cloud: Cloud name.
        cloud: String,
    },

    /// InvalidHeadersError: Provided headers are not valid. Expects JSON.
    #[snafu(display("Failed to parse headers: {}", json_error))]
    InvalidHeadersError {
        /// json_error: Error object for JSON parsing error.
        json_error: serde_json::Error,
    },

    /// InvalidMetricError: Provided metric is not valid.
    #[snafu(display("Invalid metric '{}'.", metric))]
    InvalidMetricError {
        /// metric: Metric name.
        metric: String,
    },

    /// ListCollectionsError: Failed to list indexes.
    #[snafu(display("Failed to list collections."))]
    ListCollectionsError {
        /// openapi_error: Error object for OpenAPI error.
        openapi_error: OpenAPIError<ListCollectionsError>,
    },

    /// ListIndexesError: Failed to list indexes.
    #[snafu(display("Failed to list indexes."))]
    ListIndexesError {
        /// openapi_error: Error object for OpenAPI error.
        openapi_error: OpenAPIError<ListIndexesError>,
    },

    /// MissingDimensionError: Index dimension is missing.
    #[snafu(display("Dimension missing."))]
    MissingDimensionError,

    /// MissingNameError: Index name is missing.
    #[snafu(display("Index name missing."))]
    MissingNameError,

    /// MissingSpecError: Index spec is missing.
    #[snafu(display("Spec missing."))]
    MissingSpecError,

    /// TimeoutError: Request timed out.
    #[snafu(display("Request timed out."))]
    TimeoutError,
}
