use crate::openapi::apis::{Error as OpenApiError, ResponseContent};
use anyhow::Error as AnyhowError;
use reqwest::{self, StatusCode};
use thiserror::Error;

/// PineconeError is the error type for all Pinecone SDK errors.
#[derive(Error, Debug)]
pub enum PineconeError {
    /// UnknownResponseError: Unknown response error.
    #[error("Unknown response error: status: {status}, message: {message}")]
    UnknownResponseError {
        /// status code
        status: StatusCode,
        /// message
        message: String,
    },

    /// ActionForbiddenError: Action is forbidden.
    #[error("Action forbidden error: {source}")]
    ActionForbiddenError {
        /// Source error
        source: WrappedResponseContent,
    },

    /// APIKeyMissingError: API key is not provided as an argument nor in the environment variable `PINECONE_API_KEY`.
    #[error("API key missing error: {message}")]
    APIKeyMissingError {
        /// Error message.
        message: String,
    },

    /// InvalidHeadersError: Provided headers are not valid. Expects JSON.
    #[error("Invalid headers error: {message}")]
    InvalidHeadersError {
        /// Error message.
        message: String,
    },

    /// TimeoutError: Request timed out.
    #[error("Timeout error: {message}")]
    TimeoutError {
        /// Error message.
        message: String,
    },

    /// ConnectionError: Failed to establish a connection.
    #[error("Connection error: {source}")]
    ConnectionError {
        /// Source of the error.
        source: AnyhowError,
    },

    /// ReqwestError: Error caused by Reqwest
    #[error("Reqwest error: {source}")]
    ReqwestError {
        /// Source of the error.
        source: AnyhowError,
    },

    /// SerdeError: Error caused by Serde
    #[error("Serde error: {source}")]
    SerdeError {
        /// Source of the error.
        source: AnyhowError,
    },

    /// IoError: Error caused by IO
    #[error("IO error: {message}")]
    IoError {
        /// Error message.
        message: String,
    },

    /// BadRequestError: Bad request. The request body included invalid request parameters
    #[error("Bad request error: {source}")]
    BadRequestError {
        /// Source error
        source: WrappedResponseContent,
    },

    /// UnauthorizedError: Unauthorized. Possibly caused by invalid API key
    #[error("Unauthorized error: {source}")]
    UnauthorizedError {
        /// Source error
        source: WrappedResponseContent,
    },

    /// PodQuotaExceededError: Pod quota exceeded
    #[error("Pod quota exceeded error: {source}")]
    PodQuotaExceededError {
        /// Source error
        source: WrappedResponseContent,
    },

    /// CollectionsQuotaExceededError: Collections quota exceeded
    #[error("Collections quota exceeded error: {source}")]
    CollectionsQuotaExceededError {
        /// Source error
        source: WrappedResponseContent,
    },

    /// InvalidCloudError: Provided cloud is not valid.
    #[error("Invalid cloud error: {source}")]
    InvalidCloudError {
        /// Source error
        source: WrappedResponseContent,
    },

    /// InvalidRegionError: Provided region is not valid.
    #[error("Invalid region error: {source}")]
    InvalidRegionError {
        /// Source error
        source: WrappedResponseContent,
    },

    /// InvalidConfigurationError: Provided configuration is not valid.
    #[error("Invalid configuration error: {message}")]
    InvalidConfigurationError {
        /// Error message.
        message: String,
    },

    /// CollectionNotFoundError: Collection of given name does not exist
    #[error("Collection not found error: {source}")]
    CollectionNotFoundError {
        /// Source error
        source: WrappedResponseContent,
    },

    /// IndexNotFoundError: Index of given name does not exist
    #[error("Index not found error: {source}")]
    IndexNotFoundError {
        /// Source error
        source: WrappedResponseContent,
    },

    /// ResourceAlreadyExistsError: Resource of given name already exists
    #[error("Resource already exists error: {source}")]
    ResourceAlreadyExistsError {
        /// Source error
        source: WrappedResponseContent,
    },

    /// Unprocessable entity error: The request body could not be deserialized
    #[error("Unprocessable entity error: {source}")]
    UnprocessableEntityError {
        /// Source error
        source: WrappedResponseContent,
    },

    /// PendingCollectionError: There is a pending collection created from this index
    #[error("Pending collection error: {source}")]
    PendingCollectionError {
        /// Source error
        source: WrappedResponseContent,
    },

    /// InternalServerError: Internal server error
    #[error("Internal server error: {source}")]
    InternalServerError {
        /// Source error
        source: WrappedResponseContent,
    },

    /// DataPlaneError: Failed to perform a data plane operation.
    #[error("Data plane error: {status}")]
    DataPlaneError {
        /// Error status
        status: tonic::Status,
    },

    /// InferenceError: Failed to perform an inference operation.
    #[error("Inference error: {status}")]
    InferenceError {
        /// Error status
        status: tonic::Status,
    },
}

// Implement the conversion from OpenApiError to PineconeError for CreateIndexError.
impl<T> From<OpenApiError<T>> for PineconeError {
    fn from(error: OpenApiError<T>) -> Self {
        match error {
            OpenApiError::Reqwest(inner) => PineconeError::ReqwestError {
                source: inner.into(),
            },
            OpenApiError::Serde(inner) => PineconeError::SerdeError {
                source: inner.into(),
            },
            OpenApiError::Io(inner) => PineconeError::IoError {
                message: inner.to_string(),
            },
            OpenApiError::ResponseError(inner) => handle_response_error(inner.into()),
        }
    }
}

// Helper function to handle response errors
fn handle_response_error(source: WrappedResponseContent) -> PineconeError {
    let status = source.status;
    let message = source.content.clone();

    match status {
        StatusCode::BAD_REQUEST => PineconeError::BadRequestError { source },
        StatusCode::UNAUTHORIZED => PineconeError::UnauthorizedError { source },
        StatusCode::FORBIDDEN => parse_forbidden_error(source, message),
        StatusCode::NOT_FOUND => parse_not_found_error(source, message),
        StatusCode::CONFLICT => PineconeError::ResourceAlreadyExistsError { source },
        StatusCode::PRECONDITION_FAILED => PineconeError::PendingCollectionError { source },
        StatusCode::UNPROCESSABLE_ENTITY => PineconeError::UnprocessableEntityError { source },
        StatusCode::INTERNAL_SERVER_ERROR => PineconeError::InternalServerError { source },
        _ => PineconeError::UnknownResponseError { status, message },
    }
}

fn parse_not_found_error(source: WrappedResponseContent, message: String) -> PineconeError {
    if message.contains("Index") {
        PineconeError::IndexNotFoundError { source }
    } else if message.contains("Collection") {
        PineconeError::CollectionNotFoundError { source }
    } else if message.contains("region") {
        PineconeError::InvalidRegionError { source }
    } else if message.contains("cloud") {
        PineconeError::InvalidCloudError { source }
    } else {
        PineconeError::InternalServerError { source }
    }
}

fn parse_forbidden_error(source: WrappedResponseContent, message: String) -> PineconeError {
    if message.contains("Deletion protection") {
        PineconeError::ActionForbiddenError { source }
    } else if message.contains("index") {
        PineconeError::PodQuotaExceededError { source }
    } else if message.contains("Collection") {
        PineconeError::CollectionsQuotaExceededError { source }
    } else {
        PineconeError::InternalServerError { source }
    }
}

/// WrappedResponseContent is a wrapper around ResponseContent.
#[derive(Debug)]
pub struct WrappedResponseContent {
    /// status code
    pub status: reqwest::StatusCode,
    /// content
    pub content: String,
}

impl<T> From<ResponseContent<T>> for WrappedResponseContent {
    fn from(rc: ResponseContent<T>) -> Self {
        WrappedResponseContent {
            status: rc.status,
            content: rc.content,
        }
    }
}

impl std::error::Error for WrappedResponseContent {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for WrappedResponseContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "status: {} content: {}", self.status, self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::PineconeError;
    use tokio;

    fn assert_send_sync<T: Send + Sync>() {}

    #[tokio::test]
    async fn test_pinecone_error_is_send_sync() {
        assert_send_sync::<PineconeError>();
    }
}
