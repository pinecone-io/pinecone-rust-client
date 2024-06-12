use openapi::apis::{manage_indexes_api::CreateIndexError, Error as OpenAPIError};
use snafu::prelude::*;

/// PineconeError is the error type for all Pinecone SDK errors.
#[derive(Debug, Snafu)]
pub enum PineconeError {
    /// APIKeyMissingError: API key is not provided as an argument nor in the environment variable `PINECONE_API_KEY`.
    #[snafu(display("API key missing."))]
    APIKeyMissingError,

    #[snafu(display("API key missing."))]
    CreateIndexError {
        openapi_error: OpenAPIError<CreateIndexError>,
    },

    #[snafu(display("Invalid cloud '{}'.", cloud))]
    InvalidCloudError { cloud: String },

    /// InvalidHeadersError: Provided headers are not valid. Expects JSON.
    #[snafu(display("Failed to parse headers: {}", json_error))]
    InvalidHeadersError {
        /// json_error: Error object for JSON parsing error.
        json_error: serde_json::Error,
    },

    #[snafu(display("Invalid metric '{}'.", metric))]
    InvalidMetricError { metric: String },

    #[snafu(display("Invalid region."))]
    InvalidRegionError,

    #[snafu(display("Index name missing."))]
    MissingNameError,

    #[snafu(display("Dimension missing."))]
    MissingDimensionError,

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
            (PineconeError::InvalidRegionError, PineconeError::InvalidRegionError) => true,
            _ => false,
        }
    }
}
