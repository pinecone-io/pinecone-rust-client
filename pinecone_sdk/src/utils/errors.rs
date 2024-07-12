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
    PodQuotaExceededError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
    },

    /// CollectionsQuotaExceededError: Collections quota exceeded
    CollectionsQuotaExceededError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
    },

    /// InvalidCloudError: Provided cloud is not valid.
    InvalidCloudError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
    },

    /// InvalidRegionError: Provided region is not valid.
    InvalidRegionError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
    },

    /// CollectionNotFoundError: Collection of given name does not exist
    CollectionNotFoundError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
    },

    /// IndexNotFoundError: Index of given name does not exist
    IndexNotFoundError {
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

    /// CollectionAlreadyExistsError: Collection of given name already exists
    CollectionAlreadyExistsError {
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

    /// PendingCollectionError: There is a pending collection created from this index
    PendingCollectionError {
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

// Helper function to extract status/error message
fn err_handler<T>(e: OpenApiError<T>, message: String) -> PineconeError {
    match e {
        OpenApiError::Reqwest(inner) => PineconeError::ReqwestError { source: inner },
        OpenApiError::Serde(inner) => PineconeError::SerdeError { source: inner },
        OpenApiError::Io(inner) => PineconeError::IoError {
            message: inner.to_string(),
        },
        OpenApiError::ResponseError(inner) => handle_response_error(inner, message),
    }
}

// Helper function to handle response errors
fn handle_response_error<T>(e: ResponseContent<T>, message: String) -> PineconeError {
    let err_message = e.content;
    let status = e.status;
    let message = format!("{message}: {err_message}");

    match status {
        StatusCode::BAD_REQUEST => PineconeError::BadRequestError { status, message },
        StatusCode::UNAUTHORIZED => PineconeError::UnauthorizedError { status, message },
        StatusCode::FORBIDDEN => parse_quota_exceeded_error(message),
        StatusCode::NOT_FOUND => parse_not_found_error(message),
        StatusCode::CONFLICT => parse_conflict_error(message),
        StatusCode::PRECONDITION_FAILED => {
            PineconeError::PendingCollectionError { status, message }
        }
        StatusCode::UNPROCESSABLE_ENTITY => {
            PineconeError::UnprocessableEntityError { status, message }
        }
        StatusCode::INTERNAL_SERVER_ERROR => PineconeError::InternalServerError { status, message },
        _ => PineconeError::UnknownResponseError {
            status,
            message: err_message,
        },
    }
}

fn parse_not_found_error(message: String) -> PineconeError {
    if message.contains("Index") {
        PineconeError::IndexNotFoundError {
            status: StatusCode::NOT_FOUND,
            message,
        }
    } else if message.contains("Collection") {
        PineconeError::CollectionNotFoundError {
            status: StatusCode::NOT_FOUND,
            message,
        }
    } else if message.contains("region") {
        PineconeError::InvalidRegionError {
            status: StatusCode::NOT_FOUND,
            message,
        }
    } else if message.contains("cloud") {
        PineconeError::InvalidCloudError {
            status: StatusCode::NOT_FOUND,
            message,
        }
    } else {
        PineconeError::InternalServerError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message,
        }
    }
}

fn parse_conflict_error(message: String) -> PineconeError {
    if message.contains("index") {
        PineconeError::IndexAlreadyExistsError {
            status: StatusCode::CONFLICT,
            message,
        }
    } else if message.contains("collection") {
        PineconeError::CollectionAlreadyExistsError {
            status: StatusCode::CONFLICT,
            message,
        }
    } else {
        PineconeError::InternalServerError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message,
        }
    }
}

fn parse_quota_exceeded_error(message: String) -> PineconeError {
    if message.contains("index") {
        PineconeError::PodQuotaExceededError {
            status: StatusCode::FORBIDDEN,
            message,
        }
    } else if message.contains("Collection") {
        PineconeError::CollectionsQuotaExceededError {
            status: StatusCode::FORBIDDEN,
            message,
        }
    } else {
        PineconeError::InternalServerError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message,
        }
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
            PineconeError::CollectionAlreadyExistsError { status, message } => {
                write!(
                    f,
                    "Collection already exists error: status: {}, message: {}",
                    status, message
                )
            }
            PineconeError::UnprocessableEntityError { status, message } => {
                write!(
                    f,
                    "Unprocessable entity error: status: {}, message: {}",
                    status, message
                )
            }
            PineconeError::PendingCollectionError { status, message } => {
                write!(
                    f,
                    "Pending collection error: status: {}, message: {}",
                    status, message
                )
            }
            PineconeError::InternalServerError { status, message } => {
                write!(
                    f,
                    "Internal server error: status: {}, message: {}",
                    status, message
                )
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
            PineconeError::BadRequestError { status, message } => {
                write!(
                    f,
                    "Bad request error: status: {}, message: {}",
                    status, message
                )
            }
            PineconeError::UnauthorizedError { status, message } => {
                write!(
                    f,
                    "Unauthorized error: status: {}, message: {}",
                    status, message
                )
            }
            PineconeError::PodQuotaExceededError { status, message } => {
                write!(
                    f,
                    "Pod quota exceeded error: status: {}, message: {}",
                    status, message
                )
            }
            PineconeError::CollectionsQuotaExceededError { status, message } => {
                write!(
                    f,
                    "Collections quota exceeded error: status: {}, message: {}",
                    status, message
                )
            }
            PineconeError::InvalidCloudError { status, message } => {
                write!(
                    f,
                    "Invalid cloud error: status: {}, message: {}",
                    status, message
                )
            }
            PineconeError::InvalidRegionError { status, message } => {
                write!(
                    f,
                    "Invalid region error: status: {}, message: {}",
                    status, message
                )
            }
            PineconeError::CollectionNotFoundError { status, message } => {
                write!(
                    f,
                    "Collection not found error: status: {}, message: {}",
                    status, message
                )
            }
            PineconeError::IndexNotFoundError { status, message } => {
                write!(
                    f,
                    "Index not found error: status: {}, message: {}",
                    status, message
                )
            }
            PineconeError::IndexAlreadyExistsError { status, message } => {
                write!(
                    f,
                    "Index already exists error: status: {}, message: {}",
                    status, message
                )
            }
            PineconeError::APIKeyMissingError { message } => {
                write!(f, "API key missing error: {}", message)
            }
            PineconeError::InvalidHeadersError { message } => {
                write!(f, "Invalid headers error: message: {}", message)
            }
            PineconeError::TimeoutError { message } => {
                write!(f, "Timeout error: message: {}", message)
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
            PineconeError::BadRequestError {
                status: _,
                message: _,
            } => None,
            PineconeError::UnauthorizedError {
                status: _,
                message: _,
            } => None,
            PineconeError::PodQuotaExceededError {
                status: _,
                message: _,
            } => None,
            PineconeError::CollectionsQuotaExceededError {
                status: _,
                message: _,
            } => None,
            PineconeError::InvalidCloudError {
                status: _,
                message: _,
            } => None,
            PineconeError::InvalidRegionError {
                status: _,
                message: _,
            } => None,
            PineconeError::CollectionNotFoundError {
                status: _,
                message: _,
            } => None,
            PineconeError::IndexNotFoundError {
                status: _,
                message: _,
            } => None,
            PineconeError::IndexAlreadyExistsError {
                status: _,
                message: _,
            } => None,
            PineconeError::CollectionAlreadyExistsError {
                status: _,
                message: _,
            } => None,
            PineconeError::UnprocessableEntityError {
                status: _,
                message: _,
            } => None,
            PineconeError::PendingCollectionError {
                status: _,
                message: _,
            } => None,
            PineconeError::InternalServerError {
                status: _,
                message: _,
            } => None,
            PineconeError::APIKeyMissingError { message: _ } => None,
            PineconeError::InvalidHeadersError { message: _ } => None,
            PineconeError::TimeoutError { message: _ } => None,
        }
    }
}
