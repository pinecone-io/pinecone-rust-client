use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// Vector type of an index.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Display, EnumString, Serialize, Deserialize,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum VectorType {
    /// Dense vector type
    #[default]
    Dense,
    /// Sparse vector type
    Sparse,
    /// Text (full-text search) type
    Text,
}
