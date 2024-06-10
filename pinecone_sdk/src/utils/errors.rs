use snafu::prelude::*;

/// PineconeError is the error type for all Pinecone SDK errors.
#[derive(Debug, Snafu)]
pub enum PineconeError {
    /// APIKeyMissingError: API key is not provided as an argument nor in the environment variable `PINECONE_API_KEY`.
    #[snafu(display("API key missing."))]
    APIKeyMissingError,

    /// InvalidHeadersError: Provided headers are not valid. Expects JSON.
    #[snafu(display("Failed to parse headers: {}", json_error))]
    InvalidHeadersError {
        /// json_error: Error object for JSON parsing error.
        json_error: serde_json::Error,
    },
}
