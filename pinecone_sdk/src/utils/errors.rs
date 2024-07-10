use openapi::apis::{Error as OpenApiError, ResponseContent};
use reqwest::{self, StatusCode};

/// PineconeError is the error type for all Pinecone SDK errors.
#[derive(Debug)]
pub enum PineconeError {
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
        OpenApiError::Reqwest(inner) => PineconeError::ReqwestError {
            status: inner.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            message: inner.to_string(),
        },
        OpenApiError::Serde(inner) => PineconeError::SerdeError {
            message: inner.to_string(),
        },
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
        _ => PineconeError::ReqwestError { status, message },
    }
}

fn parse_not_found_error(message: String) -> PineconeError {
    if message.contains("Index") {
        PineconeError::IndexNotFoundError {
            status: StatusCode::NOT_FOUND,
            message,
        }
    } else if message.contains("cloud") {
        PineconeError::InvalidCloudError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message,
        }
    } else if message.contains("region") {
        PineconeError::InvalidRegionError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
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
    if message.contains("Index") {
        PineconeError::IndexAlreadyExistsError {
            status: StatusCode::CONFLICT,
            message,
        }
    } else if message.contains("Collection") {
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
    if message.contains("Pod") {
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
