use openapi::apis::{Error as OpenApiError, ResponseContent};
use reqwest::{self, StatusCode};
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

    /// InvalidRegionError: Provided region is not valid.
    #[snafu(display("Invalid region."))]
    InvalidRegionError {
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
        message: String,
    },

    // new errors
    /// ReqwestError: Error caused by Reqwest
    ReqwestError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
    },

    /// SerdeError: Error caused by Serde
    SerdeError {
        /// Error message.
        message: String,
    },

    /// IoError: Error caused by IO
    IoError {
        /// Error message.
        message: String,
    },

    /// BadRequestError: Bad request. The request body included invalid request parameters
    BadRequestError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
    },

    /// UnauthorizedError: Unauthorized. Possibly caused by invalid API key
    UnauthorizedError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
    },

    /// PodQuotaExceededError: Pod quota exceeded
    QuotaExceededError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
    },

    /// IndexAlreadyExistsError: Index of given name already exists
    IndexAlreadyExistsError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
    },

    /// Unprocessable entity error: The request body could not be deserialized
    UnprocessableEntityError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
    },

    /// InternalServerError: Internal server error
    InternalServerError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
    },
}

// Implement the conversion from OpenApiError to PineconeError for CreateIndexError.
impl<T> From<(OpenApiError<T>, String)> for PineconeError {
    fn from((error, message): (OpenApiError<T>, String)) -> Self {
        err_handler(error, message)
    }
}

// TODO: implement all other From<OpenApiError> for PineconeError?

// Helper function to extract status/error message
fn err_handler<T>(e: OpenApiError<T>, message: String) -> PineconeError {
    match e {
        OpenApiError::Reqwest(e) => PineconeError::ReqwestError {
            status: match e.status() {
                Some(status) => status,
                None => StatusCode::INTERNAL_SERVER_ERROR,
            },
            message: e.to_string(),
        },
        OpenApiError::Serde(e) => PineconeError::SerdeError {
            message: e.to_string(),
        },
        OpenApiError::Io(e) => PineconeError::IoError {
            message: e.to_string(),
        },
        OpenApiError::ResponseError(e) => handle_response_error(e, message),
    }
}

fn handle_response_error<T>(e: ResponseContent<T>, message: String) -> PineconeError {
    let err_message = e.content;
    let status = e.status;
    let message = format!("{message}: {err_message}");

    match status {
        StatusCode::BAD_REQUEST => PineconeError::BadRequestError { status, message },
        StatusCode::UNAUTHORIZED => PineconeError::UnauthorizedError { status, message },
        StatusCode::UNPROCESSABLE_ENTITY => {
            PineconeError::UnprocessableEntityError { status, message }
        }
        StatusCode::INTERNAL_SERVER_ERROR => PineconeError::InternalServerError { status, message },
        _ => PineconeError::ReqwestError { status, message },
    }
}
