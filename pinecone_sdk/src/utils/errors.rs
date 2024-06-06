use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum PineconeError {
    #[snafu(display("API key missing."))]
    APIKeyMissingError,

    #[snafu(display("Failed to parse headers: {}", json_error))]
    InvalidHeadersError { json_error: serde_json::Error },
}
