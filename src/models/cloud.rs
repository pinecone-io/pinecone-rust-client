use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// The public cloud where you would like your index hosted.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Display, EnumString, Serialize, Deserialize,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Cloud {
    /// GCP
    #[default]
    Gcp,
    /// AWS
    Aws,
    /// Azure
    Azure,
}

impl From<crate::openapi::models::serverless_spec::Cloud> for Cloud {
    fn from(cloud: crate::openapi::models::serverless_spec::Cloud) -> Self {
        match cloud {
            crate::openapi::models::serverless_spec::Cloud::Gcp => Cloud::Gcp,
            crate::openapi::models::serverless_spec::Cloud::Aws => Cloud::Aws,
            crate::openapi::models::serverless_spec::Cloud::Azure => Cloud::Azure,
        }
    }
}

impl From<Cloud> for crate::openapi::models::serverless_spec::Cloud {
    fn from(cloud: Cloud) -> Self {
        match cloud {
            Cloud::Gcp => crate::openapi::models::serverless_spec::Cloud::Gcp,
            Cloud::Aws => crate::openapi::models::serverless_spec::Cloud::Aws,
            Cloud::Azure => crate::openapi::models::serverless_spec::Cloud::Azure,
        }
    }
}

impl From<crate::openapi::models::create_index_for_model_request::Cloud> for Cloud {
    fn from(cloud: crate::openapi::models::create_index_for_model_request::Cloud) -> Self {
        match cloud {
            crate::openapi::models::create_index_for_model_request::Cloud::Gcp => Cloud::Gcp,
            crate::openapi::models::create_index_for_model_request::Cloud::Aws => Cloud::Aws,
            crate::openapi::models::create_index_for_model_request::Cloud::Azure => Cloud::Azure,
        }
    }
}

impl From<Cloud> for crate::openapi::models::create_index_for_model_request::Cloud {
    fn from(cloud: Cloud) -> Self {
        match cloud {
            Cloud::Gcp => crate::openapi::models::create_index_for_model_request::Cloud::Gcp,
            Cloud::Aws => crate::openapi::models::create_index_for_model_request::Cloud::Aws,
            Cloud::Azure => crate::openapi::models::create_index_for_model_request::Cloud::Azure,
        }
    }
}
