use openapi::apis::{Error as OpenApiError, ResponseContent};

use reqwest::{self, StatusCode};

pub struct WrappedResponseContent<T> {
    pub response_content: ResponseContent<T>,
}

impl<T> From<ResponseContent<T>> for WrappedResponseContent<T> {
    fn from(response_content: ResponseContent<T>) -> Self {
        WrappedResponseContent {
            response_content: ResponseContent {
                status: response_content.status,
                content: response_content.content,
                entity: response_content.entity,
            },
        }
    }
}

impl<T> std::error::Error for WrappedResponseContent<T> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl<T> std::fmt::Display for WrappedResponseContent<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "status: {} content: {}",
            self.response_content.status, self.response_content.content
        )
    }
}

impl<T> std::fmt::Debug for WrappedResponseContent<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "status: {} content: {}",
            self.response_content.status, self.response_content.content
        )
    }
}

/// PineconeError is the error type for all Pinecone SDK errors.
#[derive(Debug)]
pub enum PineconeError<T> {
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
        /// error
        source: WrappedResponseContent<T>,
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
impl<T: std::fmt::Display> From<(OpenApiError<T>, String)> for PineconeError<T> {
    fn from((error, message): (OpenApiError<T>, String)) -> Self {
        err_handler(error, message)
    }
}

// Helper function to extract status/error message
fn err_handler<T: std::fmt::Display>(e: OpenApiError<T>, message: String) -> PineconeError<T> {
    match e {
        OpenApiError::Reqwest(inner) => PineconeError::<T>::ReqwestError { source: inner },
        OpenApiError::Serde(inner) => PineconeError::<T>::SerdeError { source: inner },
        OpenApiError::Io(inner) => PineconeError::<T>::IoError {
            message: inner.to_string(),
        },
        OpenApiError::ResponseError(inner) => handle_response_error(inner, message),
    }
}

// Helper function to handle response errors
fn handle_response_error<T: std::fmt::Display>(
    e: ResponseContent<T>,
    message: String,
) -> PineconeError<T> {
    // let err_message = e.content.;
    let status = e.status;
    let message = format!("{message}: {}", e.content);

    match status {
        StatusCode::BAD_REQUEST => PineconeError::BadRequestError { source: e.into() },
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
            message: e.content,
        },
    }
}

fn parse_not_found_error<T: std::fmt::Display>(message: String) -> PineconeError<T> {
    if message.contains("Index") {
        PineconeError::<T>::IndexNotFoundError {
            status: StatusCode::NOT_FOUND,
            message,
        }
    } else if message.contains("Collection") {
        PineconeError::<T>::CollectionNotFoundError {
            status: StatusCode::NOT_FOUND,
            message,
        }
    } else if message.contains("region") {
        PineconeError::<T>::InvalidRegionError {
            status: StatusCode::NOT_FOUND,
            message,
        }
    } else if message.contains("cloud") {
        PineconeError::<T>::InvalidCloudError {
            status: StatusCode::NOT_FOUND,
            message,
        }
    } else {
        PineconeError::<T>::InternalServerError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message,
        }
    }
}

fn parse_conflict_error<T: std::fmt::Display>(message: String) -> PineconeError<T> {
    if message.contains("index") {
        PineconeError::<T>::IndexAlreadyExistsError {
            status: StatusCode::CONFLICT,
            message,
        }
    } else if message.contains("collection") {
        PineconeError::<T>::CollectionAlreadyExistsError {
            status: StatusCode::CONFLICT,
            message,
        }
    } else {
        PineconeError::<T>::InternalServerError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message,
        }
    }
}

fn parse_quota_exceeded_error<T: std::fmt::Display>(message: String) -> PineconeError<T> {
    if message.contains("index") {
        PineconeError::<T>::PodQuotaExceededError {
            status: StatusCode::FORBIDDEN,
            message,
        }
    } else if message.contains("Collection") {
        PineconeError::<T>::CollectionsQuotaExceededError {
            status: StatusCode::FORBIDDEN,
            message,
        }
    } else {
        PineconeError::<T>::InternalServerError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message,
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for PineconeError<T> {
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
            PineconeError::BadRequestError { source } => {
                write!(f, "Bad request error: {}", source)
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

impl<T: std::fmt::Display + std::fmt::Debug + 'static> std::error::Error for PineconeError<T> {
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
