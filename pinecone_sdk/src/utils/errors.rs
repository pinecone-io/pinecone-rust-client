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

    #[snafu(display("Failed to parse headers: {}", json_error))]
    InvalidHeadersError { json_error: serde_json::Error },

    #[snafu(display("Invalid cloud '{}'.", cloud))]
    InvalidCloudError { cloud: String },

    #[snafu(display("Invalid metric '{}'.", metric))]
    InvalidMetricError { metric: String },

    #[snafu(display("Invalid region."))]
    InvalidRegionError,
}
