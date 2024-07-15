use openapi::apis::{Error as OpenApiError, ResponseContent};

use reqwest::{self, StatusCode};

/// PineconeError is the error type for all Pinecone SDK errors.
#[derive(Debug)]
pub enum PineconeError {
    /// UnknownResponseError: Unknown response error.
    UnknownResponseError {
        /// status code
        status: StatusCode,
        /// message
        message: String,
    },
    /// APIKeyMissingError: API key is not provided as an argument nor in the environment variable `PINECONE_API_KEY`.
    APIKeyMissingError {
        /// Error message.
        message: String,
    },

    /// InvalidHeadersError: Provided headers are not valid. Expects JSON.
    InvalidHeadersError {
        /// Error message.
        message: String,
    },

    /// TimeoutError: Request timed out.
    TimeoutError {
        /// Error message.
        message: String,
    },

    /// ConnectionError: Failed to establish a connection.
    ConnectionError {
        /// inner: Error object for connection error.
        source: Box<dyn std::error::Error>,
    },

    /// ReqwestError: Error caused by Reqwest
    ReqwestError {
        /// Source error
        source: reqwest::Error,
    },

    /// SerdeError: Error caused by Serde
    SerdeError {
        /// Source of the error.
        source: serde_json::Error,
    },

    /// IoError: Error caused by IO
    IoError {
        /// Error message.
        message: String,
    },

    /// BadRequestError: Bad request. The request body included invalid request parameters
    BadRequestError {
        /// error
        source: WrappedResponseContent,
    },

    /// UnauthorizedError: Unauthorized. Possibly caused by invalid API key
    UnauthorizedError {
        /// error
        source: WrappedResponseContent,
    },

    /// PodQuotaExceededError: Pod quota exceeded
    PodQuotaExceededError {
        /// error
        source: WrappedResponseContent,
    },

    /// CollectionsQuotaExceededError: Collections quota exceeded
    CollectionsQuotaExceededError {
        /// error
        source: WrappedResponseContent,
    },

    /// InvalidCloudError: Provided cloud is not valid.
    InvalidCloudError {
        /// error
        source: WrappedResponseContent,
    },

    /// InvalidRegionError: Provided region is not valid.
    InvalidRegionError {
        /// error
        source: WrappedResponseContent,
    },

    /// CollectionNotFoundError: Collection of given name does not exist
    CollectionNotFoundError {
        /// error
        source: WrappedResponseContent,
    },

    /// IndexNotFoundError: Index of given name does not exist
    IndexNotFoundError {
        /// error
        source: WrappedResponseContent,
    },

    /// ResourceAlreadyExistsError: Resource of given name already exists
    ResourceAlreadyExistsError {
        /// error
        source: WrappedResponseContent,
    },

    /// Unprocessable entity error: The request body could not be deserialized
    UnprocessableEntityError {
        /// error
        source: WrappedResponseContent,
    },

    /// PendingCollectionError: There is a pending collection created from this index
    PendingCollectionError {
        /// error
        source: WrappedResponseContent,
    },

    /// InternalServerError: Internal server error
    InternalServerError {
        /// error
        source: WrappedResponseContent,
    },

    /// UpsertError: Failed to upsert data.
    UpsertError {
        /// source: Error object for tonic error.
        source: tonic::Status,
    },
}

// Implement the conversion from OpenApiError to PineconeError for CreateIndexError.
impl<T> From<OpenApiError<T>> for PineconeError {
    fn from(error: OpenApiError<T>) -> Self {
        match error {
            OpenApiError::Reqwest(inner) => PineconeError::ReqwestError { source: inner },
            OpenApiError::Serde(inner) => PineconeError::SerdeError { source: inner },
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
        StatusCode::FORBIDDEN => parse_quota_exceeded_error(source, message),
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

fn parse_quota_exceeded_error(source: WrappedResponseContent, message: String) -> PineconeError {
    if message.contains("index") {
        PineconeError::PodQuotaExceededError { source }
    } else if message.contains("Collection") {
        PineconeError::CollectionsQuotaExceededError { source }
    } else {
        PineconeError::InternalServerError { source }
    }
}

impl std::fmt::Display for PineconeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PineconeError::UnknownResponseError { status, message } => {
                write!(
                    f,
                    "Unknown response error: status: {}, message: {}",
                    status, message
                )
            }
            PineconeError::ResourceAlreadyExistsError { source } => {
                write!(f, "Resource already exists error: {}", source)
            }
            PineconeError::UnprocessableEntityError { source } => {
                write!(f, "Unprocessable entity error: {}", source)
            }
            PineconeError::PendingCollectionError { source } => {
                write!(f, "Pending collection error: {}", source)
            }
            PineconeError::InternalServerError { source } => {
                write!(f, "Internal server error: {}", source)
            }
            PineconeError::ReqwestError { source } => {
                write!(f, "Reqwest error: {}", source.to_string())
            }
            PineconeError::SerdeError { source } => {
                write!(f, "Serde error: {}", source.to_string())
            }
            PineconeError::IoError { message } => {
                write!(f, "IO error: {}", message)
            }
            PineconeError::BadRequestError { source } => {
                write!(f, "Bad request error: {}", source)
            }
            PineconeError::UnauthorizedError { source } => {
                write!(f, "Unauthorized error: status: {}", source)
            }
            PineconeError::PodQuotaExceededError { source } => {
                write!(f, "Pod quota exceeded error: {}", source)
            }
            PineconeError::CollectionsQuotaExceededError { source } => {
                write!(f, "Collections quota exceeded error: {}", source)
            }
            PineconeError::InvalidCloudError { source } => {
                write!(f, "Invalid cloud error: status: {}", source)
            }
            PineconeError::InvalidRegionError { source } => {
                write!(f, "Invalid region error: {}", source)
            }
            PineconeError::CollectionNotFoundError { source } => {
                write!(f, "Collection not found error: {}", source)
            }
            PineconeError::IndexNotFoundError { source } => {
                write!(f, "Index not found error: status: {}", source)
            }
            PineconeError::APIKeyMissingError { message } => {
                write!(f, "API key missing error: {}", message)
            }
            PineconeError::InvalidHeadersError { message } => {
                write!(f, "Invalid headers error: {}", message)
            }
            PineconeError::TimeoutError { message } => {
                write!(f, "Timeout error: {}", message)
            }
            PineconeError::ConnectionError { source } => {
                write!(f, "Connection error: {}", source)
            }
            PineconeError::UpsertError { source } => {
                write!(f, "Upsert error: {}", source)
            }
        }
    }
}

impl std::error::Error for PineconeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PineconeError::UnknownResponseError {
                status: _,
                message: _,
            } => None,
            PineconeError::ReqwestError { source } => Some(source),
            PineconeError::SerdeError { source } => Some(source),
            PineconeError::IoError { message: _ } => None,
            PineconeError::BadRequestError { source } => Some(source),
            PineconeError::UnauthorizedError { source } => Some(source),
            PineconeError::PodQuotaExceededError { source } => Some(source),
            PineconeError::CollectionsQuotaExceededError { source } => Some(source),
            PineconeError::InvalidCloudError { source } => Some(source),
            PineconeError::InvalidRegionError { source } => Some(source),
            PineconeError::CollectionNotFoundError { source } => Some(source),
            PineconeError::IndexNotFoundError { source } => Some(source),
            PineconeError::ResourceAlreadyExistsError { source } => Some(source),
            PineconeError::UnprocessableEntityError { source } => Some(source),
            PineconeError::PendingCollectionError { source } => Some(source),
            PineconeError::InternalServerError { source } => Some(source),
            PineconeError::APIKeyMissingError { message: _ } => None,
            PineconeError::InvalidHeadersError { message: _ } => None,
            PineconeError::TimeoutError { message: _ } => None,
            PineconeError::ConnectionError { source } => Some(source.as_ref()),
            PineconeError::UpsertError { source } => Some(source),
        }
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
