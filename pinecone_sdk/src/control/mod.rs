/// Module for creating an index.
pub mod create_index;

/// Module for listing all indexes.
pub mod list_indexes;

pub use openapi::models::create_index_request::Metric;
pub use openapi::models::serverless_spec::Cloud;