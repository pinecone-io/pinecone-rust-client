use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Display, EnumString, Serialize, Deserialize,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum VectorType {
    #[default]
    Dense,
    Sparse,
}
