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
        status: Option<reqwest::StatusCode>,
        msg: String,
    },

    /// CreateCollectionError: Failed to create a collection.
    #[snafu(display("Failed to create collection: {}", msg))]
    CreateCollectionError {
        status: Option<reqwest::StatusCode>,
        msg: String,
    },

    /// CreateIndexError: Failed to create an index.
    #[snafu(display("Failed to create an index: {}", msg))]
    CreateIndexError {
        status: Option<reqwest::StatusCode>,
        msg: String,
    },

    /// DeleteCollectionError: Failed to delete an index.
    #[snafu(display("Failed to delete collection: {}", msg))]
    DeleteCollectionError {
        status: Option<reqwest::StatusCode>,
        msg: String,
    },

    /// DeleteIndexError: Failed to delete an index.
    #[snafu(display("Failed to delete index: {}", msg))]
    DeleteIndexError {
        status: Option<reqwest::StatusCode>,
        msg: String,
    },

    /// DescribeIndexError: Failed to describe an index.
    #[snafu(display("Failed to describe the index"))]
    DescribeIndexError {
        status: Option<reqwest::StatusCode>,
        msg: String,
    },

    /// InvalidCloudError: Provided cloud is not valid.
    #[snafu(display("Invalid cloud."))]
    InvalidCloudError {
        status: Option<reqwest::StatusCode>,
        msg: String,
    },

    /// InvalidHeadersError: Provided headers are not valid. Expects JSON.
    #[snafu(display("Failed to parse headers."))]
    InvalidHeadersError {
        status: Option<reqwest::StatusCode>,
        msg: String,
    },

    /// InvalidMetricError: Provided metric is not valid.
    #[snafu(display("Invalid metric."))]
    InvalidMetricError { msg: String },

    /// ListCollectionsError: Failed to list indexes.
    #[snafu(display("Failed to list collections: {}", msg))]
    ListCollectionsError {
        status: Option<reqwest::StatusCode>,
        msg: String,
    },

    /// ListIndexesError: Failed to list indexes.
    #[snafu(display("Failed to list indexes: {}", msg))]
    ListIndexesError {
        status: Option<reqwest::StatusCode>,
        msg: String,
    },

    /// MissingDimensionError: Index dimension is missing.
    #[snafu(display("Dimension missing."))]
    MissingDimensionError { msg: String },

    /// MissingNameError: Index name is missing.
    #[snafu(display("Index name missing."))]
    MissingNameError { msg: String },

    /// MissingSpecError: Index spec is missing.
    #[snafu(display("Spec missing."))]
    MissingSpecError { msg: String },

    /// TimeoutError: Request timed out.
    #[snafu(display("Request timed out."))]
    TimeoutError { msg: String },
}
