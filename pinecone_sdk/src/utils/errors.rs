use openapi::apis::manage_indexes_api::CreateIndexError;
use openapi::apis::Error as OpenApiError;
use reqwest;
use snafu::Snafu;

/// PineconeError is the error type for all Pinecone SDK errors.
#[derive(Debug, Snafu)]
pub enum PineconeError {
    /// APIKeyMissingError: API key is not provided as an argument nor in the environment variable `PINECONE_API_KEY`.
    #[snafu(display("API Key is missing"))]
    APIKeyMissingError,

    /// ConfigureIndexError: Failed to configure an index.
    #[snafu(display("Failed to configure index: {}", msg))]
    ConfigureIndexError {
        /// HTTP status code.
        status: Option<reqwest::StatusCode>,
        /// Error message.
        msg: String,
    },

    /// CreateCollectionError: Failed to create a collection.
    #[snafu(display("Failed to create collection: {}", msg))]
    CreateCollectionError {
        /// HTTP status code.
        status: Option<reqwest::StatusCode>,
        /// Error message.
        msg: String,
    },

    /// CreateIndexError: Failed to create an index.
    #[snafu(display("Failed to create an index: {}", msg))]
    CreateIndexError {
        /// HTTP status code.
        status: Option<reqwest::StatusCode>,
        /// Error message.
        msg: String,
    },

    /// DeleteCollectionError: Failed to delete an index.
    #[snafu(display("Failed to delete collection: {}", msg))]
    DeleteCollectionError {
        /// HTTP status code.
        status: Option<reqwest::StatusCode>,
        /// Error message.
        msg: String,
    },

    /// DeleteIndexError: Failed to delete an index.
    #[snafu(display("Failed to delete index: {}", msg))]
    DeleteIndexError {
        /// HTTP status code.
        status: Option<reqwest::StatusCode>,
        /// Error message.
        msg: String,
    },

    /// DescribeIndexError: Failed to describe an index.
    #[snafu(display("Failed to describe the index"))]
    DescribeIndexError {
        /// HTTP status code.
        status: Option<reqwest::StatusCode>,
        /// Error message.
        msg: String,
    },

    /// InvalidCloudError: Provided cloud is not valid.
    #[snafu(display("Invalid cloud."))]
    InvalidCloudError {
        /// HTTP status code.
        status: Option<reqwest::StatusCode>,
        /// Error message.
        msg: String,
    },

    /// InvalidHeadersError: Provided headers are not valid. Expects JSON.
    #[snafu(display("Failed to parse headers."))]
    InvalidHeadersError {
        /// HTTP status code.
        status: Option<reqwest::StatusCode>,
        /// Error message.
        msg: String,
    },

    /// InvalidMetricError: Provided metric is not valid.
    #[snafu(display("Invalid metric."))]
    InvalidMetricError {
        /// Error message.
        msg: String,
    },

    /// ListCollectionsError: Failed to list indexes.
    #[snafu(display("Failed to list collections: {}", msg))]
    ListCollectionsError {
        /// HTTP status code.
        status: Option<reqwest::StatusCode>,
        /// Error message.
        msg: String,
    },

    /// ListIndexesError: Failed to list indexes.
    #[snafu(display("Failed to list indexes: {}", msg))]
    ListIndexesError {
        /// HTTP status code.
        status: Option<reqwest::StatusCode>,
        /// Error message.
        msg: String,
    },

    /// MissingDimensionError: Index dimension is missing.
    #[snafu(display("Dimension missing."))]
    MissingDimensionError {
        /// Error message.
        msg: String,
    },

    /// MissingNameError: Index name is missing.
    #[snafu(display("Index name missing."))]
    MissingNameError {
        /// Error message.
        msg: String,
    },

    /// MissingSpecError: Index spec is missing.
    #[snafu(display("Spec missing."))]
    MissingSpecError {
        /// Error message.
        msg: String,
    },

    /// TimeoutError: Request timed out.
    #[snafu(display("Request timed out."))]
    TimeoutError {
        /// Error message.
        msg: String,
    },
}

// Implement the conversion from OpenApiError to PineconeError for CreateIndexError.
impl From<OpenApiError<CreateIndexError>> for PineconeError {
    fn from(error: OpenApiError<CreateIndexError>) -> Self {
        let (status, msg) = get_err_elements(error);
        PineconeError::CreateIndexError { status, msg }
    }
}

// TODO: implement all other From<OpenApiError> for PineconeError?

// Helper function to extract status/error message
fn get_err_elements<T>(e: openapi::apis::Error<T>) -> (Option<reqwest::StatusCode>, String) {
    match e {
        openapi::apis::Error::Reqwest(e) => (e.status(), e.to_string()),
        openapi::apis::Error::Serde(e) => (None, e.to_string()),
        openapi::apis::Error::Io(e) => (None, e.to_string()),
        openapi::apis::Error::ResponseError(e) => (Some(e.status), e.content),
    }
}
