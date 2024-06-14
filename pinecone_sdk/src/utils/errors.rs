use openapi::apis::{manage_indexes_api::{CreateIndexError, ListIndexesError}, Error as OpenAPIError};
use snafu::prelude::*;

/// PineconeError is the error type for all Pinecone SDK errors.
#[derive(Debug, Snafu)]
pub enum PineconeError {
    /// APIKeyMissingError: API key is not provided as an argument nor in the environment variable `PINECONE_API_KEY`.
    #[snafu(display("API key missing."))]
    APIKeyMissingError,

    /// CreateIndexError: Failed to create an index.
    #[snafu(display("API key missing."))]
    CreateIndexError {
        /// openapi_error: Error object for OpenAPI error.
        openapi_error: OpenAPIError<CreateIndexError>,
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

    /// ListIndexesError: Failed to list indexes.
    #[snafu(display("Failed to list indexes."))]
    ListIndexesError {
        /// openapi_error: Error object for OpenAPI error.
        openapi_error: OpenAPIError<ListIndexesError>,
    },

    /// MissingNameError: Index name is missing.
    #[snafu(display("Index name missing."))]
    MissingNameError,

    /// MissingDimensionError: Index dimension is missing.
    #[snafu(display("Dimension missing."))]
    MissingDimensionError,

    /// MissingSpecError: Index spec is missing.
    #[snafu(display("Spec missing."))]
    MissingSpecError,
}

impl PartialEq for PineconeError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PineconeError::APIKeyMissingError, PineconeError::APIKeyMissingError) => true,
            (PineconeError::CreateIndexError { .. }, PineconeError::CreateIndexError { .. }) => {
                true
            }
            (PineconeError::MissingNameError, PineconeError::MissingNameError) => true,
            (PineconeError::MissingDimensionError, PineconeError::MissingDimensionError) => true,
            (PineconeError::MissingSpecError, PineconeError::MissingSpecError) => true,
            (
                PineconeError::InvalidHeadersError { .. },
                PineconeError::InvalidHeadersError { .. },
            ) => true,
            (PineconeError::InvalidCloudError { .. }, PineconeError::InvalidCloudError { .. }) => {
                true
            }
            (
                PineconeError::InvalidMetricError { .. },
                PineconeError::InvalidMetricError { .. },
            ) => true,
            _ => false,
        }
    }
}
