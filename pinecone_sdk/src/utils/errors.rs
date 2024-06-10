use openapi::apis::{manage_indexes_api::CreateIndexError, Error as OpenAPIError};
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum PineconeError {
    #[snafu(display("API key missing."))]
    APIKeyMissingError,

    #[snafu(display("API key missing."))]
    CreateIndexError {
        openapi_error: OpenAPIError<CreateIndexError>,
    },

    #[snafu(display("Index name missing."))]
    MissingNameError,

    #[snafu(display("Dimension missing."))]
    MissingDimensionError,

    #[snafu(display("Spec missing."))]
    MissingSpecError,

    #[snafu(display("Failed to parse headers: {}", json_error))]
    InvalidHeadersError { json_error: serde_json::Error },

    #[snafu(display("Invalid cloud '{}'.", cloud))]
    InvalidCloudError { cloud: String },

    #[snafu(display("Invalid metric '{}'.", metric))]
    InvalidMetricError { metric: String },

    #[snafu(display("Invalid region."))]
    InvalidRegionError,
}

impl PartialEq for PineconeError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PineconeError::APIKeyMissingError, PineconeError::APIKeyMissingError) => true,
            (PineconeError::CreateIndexError { .. }, PineconeError::CreateIndexError { .. }) => true,
            (PineconeError::MissingNameError, PineconeError::MissingNameError) => true,
            (PineconeError::MissingDimensionError, PineconeError::MissingDimensionError) => true,
            (PineconeError::MissingSpecError, PineconeError::MissingSpecError) => true,
            (PineconeError::InvalidHeadersError { .. }, PineconeError::InvalidHeadersError { .. }) => true,
            (PineconeError::InvalidCloudError { .. }, PineconeError::InvalidCloudError { .. }) => true,
            (PineconeError::InvalidMetricError { .. }, PineconeError::InvalidMetricError { .. }) => true,
            (PineconeError::InvalidRegionError, PineconeError::InvalidRegionError) => true,
            _ => false,
        }
    }
}