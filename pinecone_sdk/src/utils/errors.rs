use openapi::apis::{Error as OpenApiError, ResponseContent};
use reqwest::{self, StatusCode};

/// PineconeError is the error type for all Pinecone SDK errors.
#[derive(Debug)]
pub enum PineconeError<T> {
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
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
        source: reqwest::Error,
    },

    /// SerdeError: Error caused by Serde
    SerdeError {
        /// Error message.
        message: String,
        source: serde_json::Error,
    },

    /// IoError: Error caused by IO
    IoError {
        /// Error message.
        message: String,
        source: std::io::Error,
    },

    /// BadRequestError: Bad request. The request body included invalid request parameters
    BadRequestError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
        source: ResponseContent<T>,
    },

    /// UnauthorizedError: Unauthorized. Possibly caused by invalid API key
    UnauthorizedError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
        source: ResponseContent<T>,
    },

    /// PodQuotaExceededError: Pod quota exceeded
    PodQuotaExceededError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
        source: ResponseContent<T>,
    },

    /// CollectionsQuotaExceededError: Collections quota exceeded
    CollectionsQuotaExceededError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
        source: ResponseContent<T>,
    },

    /// InvalidCloudError: Provided cloud is not valid.
    InvalidCloudError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
        source: ResponseContent<T>,
    },

    /// InvalidRegionError: Provided region is not valid.
    InvalidRegionError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
        source: ResponseContent<T>,
    },

    /// CollectionNotFoundError: Collection of given name does not exist
    CollectionNotFoundError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
        source: ResponseContent<T>,
    },

    /// IndexNotFoundError: Index of given name does not exist
    IndexNotFoundError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
        source: ResponseContent<T>,
    },

    /// IndexAlreadyExistsError: Index of given name already exists
    IndexAlreadyExistsError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
        source: ResponseContent<T>,
    },

    /// CollectionAlreadyExistsError: Collection of given name already exists
    CollectionAlreadyExistsError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
        source: ResponseContent<T>,
    },

    /// Unprocessable entity error: The request body could not be deserialized
    UnprocessableEntityError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
        source: ResponseContent<T>,
    },

    /// PendingCollectionError: There is a pending collection created from this index
    PendingCollectionError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
        source: ResponseContent<T>,
    },

    /// InternalServerError: Internal server error
    InternalServerError {
        /// HTTP status code.
        status: StatusCode,
        /// Error message.
        message: String,
        source: ResponseContent<T>,
    },

    /// UnknownError: Unknown error
    UnknownError {
        /// Error message.
        message: String,
    },
}

// Implement the conversion from OpenApiError to PineconeError for CreateIndexError.
impl<T> From<(OpenApiError<T>, String)> for PineconeError<T> {
    fn from((error, message): (OpenApiError<T>, String)) -> Self {
        err_handler(error, message)
    }
}

// Helper function to extract status/error message
fn err_handler<T>(e: OpenApiError<T>, message: String) -> PineconeError<T> {
    match e {
        OpenApiError::Reqwest(inner) => PineconeError::ReqwestError {
            status: inner.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            message: inner.to_string(),
            source: inner,
        },
        OpenApiError::Serde(inner) => PineconeError::SerdeError {
            message: inner.to_string(),
            source: inner,
        },
        OpenApiError::Io(inner) => PineconeError::IoError {
            message: inner.to_string(),
            source: inner,
        },
        OpenApiError::ResponseError(inner) => handle_response_error(inner, message),
    }
}

// Helper function to handle response errors
fn handle_response_error<T>(source: ResponseContent<T>, message: String) -> PineconeError<T> {
    let err_message = source.content.clone();
    let status = source.status;
    let message = format!("{message}: {err_message}");

    match status {
        StatusCode::BAD_REQUEST => PineconeError::BadRequestError {
            status,
            message,
            source,
        },
        StatusCode::UNAUTHORIZED => PineconeError::UnauthorizedError {
            status,
            message,
            source,
        },
        StatusCode::FORBIDDEN => parse_quota_exceeded_error(message, source),
        StatusCode::NOT_FOUND => parse_not_found_error(message, source),
        StatusCode::CONFLICT => parse_conflict_error(message, source),
        StatusCode::PRECONDITION_FAILED => PineconeError::PendingCollectionError {
            status,
            message,
            source,
        },
        StatusCode::UNPROCESSABLE_ENTITY => PineconeError::UnprocessableEntityError {
            status,
            message,
            source,
        },
        StatusCode::INTERNAL_SERVER_ERROR => PineconeError::InternalServerError {
            status,
            message,
            source,
        },
        _ => PineconeError::UnknownError { message },
    }
}

fn parse_not_found_error<T>(message: String, source: ResponseContent<T>) -> PineconeError<T> {
    let status = StatusCode::NOT_FOUND;
    if message.contains("Index") {
        PineconeError::IndexNotFoundError {
            status,
            message,
            source,
        }
    } else if message.contains("Collection") {
        PineconeError::CollectionNotFoundError {
            status,
            message,
            source,
        }
    } else if message.contains("region") {
        PineconeError::InvalidRegionError {
            status,
            message,
            source,
        }
    } else if message.contains("cloud") {
        PineconeError::InvalidCloudError {
            status,
            message,
            source,
        }
    } else {
        PineconeError::UnknownError { message }
    }
}

fn parse_conflict_error<T>(message: String, source: ResponseContent<T>) -> PineconeError<T> {
    let status = StatusCode::CONFLICT;
    if message.contains("index") {
        PineconeError::IndexAlreadyExistsError {
            status,
            message,
            source,
        }
    } else if message.contains("collection") {
        PineconeError::CollectionAlreadyExistsError {
            status,
            message,
            source,
        }
    } else {
        PineconeError::UnknownError { message }
    }
}

fn parse_quota_exceeded_error<T>(message: String, source: ResponseContent<T>) -> PineconeError<T> {
    let status = StatusCode::FORBIDDEN;
    if message.contains("index") {
        PineconeError::PodQuotaExceededError {
            status,
            message,
            source,
        }
    } else if message.contains("Collection") {
        PineconeError::CollectionsQuotaExceededError {
            status,
            message,
            source,
        }
    } else {
        PineconeError::UnknownError { message }
    }
}
