use crate::openapi::apis::configuration::ApiKey;
use crate::openapi::apis::configuration::Configuration;
use crate::utils::errors::PineconeError;
use crate::utils::user_agent::get_user_agent;
use crate::version::API_VERSION;
use serde_json;
use std::collections::HashMap;

/// The `PINECONE_API_VERSION_KEY` is the key for the Pinecone API version header.
pub const PINECONE_API_VERSION_KEY: &str = "X-Pinecone-Api-Version";

/// Control plane module.
pub mod control;

/// Data plane module.
pub mod data;

/// Inference module.
pub mod inference;

/// The `PineconeClientConfig` struct takes in the parameters to configure the Pinecone client.
#[derive(Default)]
pub struct PineconeClientConfig {
    /// Pinecone API key
    pub api_key: Option<String>,
    /// The Pinecone controller host
    pub control_plane_host: Option<String>,
    /// Additional headers to be included in all requests
    pub additional_headers: Option<HashMap<String, String>>,
    /// The source tag
    pub source_tag: Option<String>,
}

impl PineconeClientConfig {
    /// The `PineconeClient` struct is the main entry point for interacting with Pinecone via this Rust SDK.
    /// It is used to create, delete, and manage your indexes and collections.
    /// This function constructs a `PineconeClient` struct using the provided configuration.
    ///
    /// ### Arguments
    /// * `api_key: Option<&str>` - The API key used for authentication.
    /// * `control_plane_host: Option<&str>` - The Pinecone controller host. Default is `https://api.pinecone.io`.
    /// * `additional_headers: Option<HashMap<String, String>>` - Additional headers to be included in all requests. Expects a HashMap. If no api version header is provided, it will be added.
    /// * `source_tag: Option<&str>` - A tag to identify the source of the request.
    ///
    /// ### Return
    /// * `Result<PineconeClient, PineconeError>`
    ///
    /// ### Configuration with environment variables
    /// If arguments are not provided, the SDK will attempt to read the following environment variables:
    /// - `PINECONE_API_KEY`: The API key used for authentication. If not passed as an argument, it will be read from the environment variable.
    /// - `PINECONE_CONTROLLER_HOST`: The Pinecone controller host. Default is `https://api.pinecone.io`.
    /// - `PINECONE_ADDITIONAL_HEADERS`: Additional headers to be included in all requests. Expects JSON.
    ///
    /// ### Example
    /// ```no_run
    /// use pinecone_sdk::pinecone::{PineconeClient, PineconeClientConfig};
    ///
    /// // Create a Pinecone client with the API key and controller host.
    /// 
    /// let config = PineconeClientConfig {
    ///     api_key: Some("INSERT_API_KEY".to_string()),
    ///     control_plane_host: Some("INSERT_CONTROLLER_HOST".to_string()),
    ///     ..Default::default()
    /// };
    /// let pinecone: PineconeClient = config.client().expect("Failed to create Pinecone instance");
    /// ```
    pub fn client(self) -> Result<PineconeClient, PineconeError> {
        // get api key
        let api_key = match self.api_key {
            Some(key) => key.to_string(),
            None => match std::env::var("PINECONE_API_KEY") {
                Ok(key) => key,
                Err(_) => {
                    let message =
                        "API key is not provided as an argument nor as an environment variable";
                    return Err(PineconeError::APIKeyMissingError {
                        message: message.to_string(),
                    });
                }
            },
        };

        let env_controller = std::env::var("PINECONE_CONTROLLER_HOST")
            .unwrap_or("https://api.pinecone.io".to_string());
        let controller_host = self.control_plane_host.unwrap_or(env_controller);

        // get user agent
        let user_agent = get_user_agent(self.source_tag.as_ref().map(|s| s.as_str()));

        // get additional headers
        let mut additional_headers =
            self.additional_headers
                .unwrap_or(match std::env::var("PINECONE_ADDITIONAL_HEADERS") {
                    Ok(headers) => match serde_json::from_str(&headers) {
                        Ok(headers) => headers,
                        Err(_) => {
                            let message = "Provided headers are not valid. Expects JSON.";
                            return Err(PineconeError::InvalidHeadersError {
                                message: message.to_string(),
                            });
                        }
                    },
                    Err(_) => HashMap::new(),
                });

        // add X-Pinecone-Api-Version header if not present
        // case insensitive
        if !additional_headers
            .keys()
            .any(|k| k.eq_ignore_ascii_case(PINECONE_API_VERSION_KEY))
        {
            add_api_version_header(&mut additional_headers);
        }

        // create reqwest headers
        let headers: reqwest::header::HeaderMap =
            (&additional_headers)
                .try_into()
                .map_err(|_| PineconeError::InvalidHeadersError {
                    message: "Provided headers are not valid".to_string(),
                })?;

        // create reqwest client with headers
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| PineconeError::ReqwestError { source: e })?;

        let openapi_config = Configuration {
            base_path: controller_host.to_string(),
            user_agent: Some(user_agent.to_string()),
            api_key: Some(ApiKey {
                prefix: None,
                key: api_key.clone(),
            }),
            client,
            ..Default::default()
        };

        // return Pinecone client
        return Ok(PineconeClient {
            api_key,
            controller_url: controller_host.to_string(),
            additional_headers,
            source_tag: self.source_tag,
            user_agent: Some(user_agent),
            openapi_config,
        });
    }
}

/// The `PineconeClient` struct is the main entry point for interacting with Pinecone via this Rust SDK.
#[derive(Debug, Clone)]
pub struct PineconeClient {
    /// Pinecone API key
    api_key: String,
    /// The Pinecone controller host
    controller_url: String,
    /// Additional headers to be included in all requests
    additional_headers: HashMap<String, String>,
    /// The source tag
    source_tag: Option<String>,
    /// The user agent
    user_agent: Option<String>,
    /// Configuration used for OpenAPI endpoint calls
    openapi_config: Configuration,
}

/// Helper function to add the API version header to the headers.
fn add_api_version_header(headers: &mut HashMap<String, String>) {
    headers.insert(
        PINECONE_API_VERSION_KEY.to_string(),
        API_VERSION.to_string(),
    );
}

impl TryFrom<PineconeClientConfig> for PineconeClient {
    type Error = PineconeError;

    fn try_from(config: PineconeClientConfig) -> Result<Self, Self::Error> {
        config.client()
    }
}

/// The `PineconeClient` struct is the main entry point for interacting with Pinecone via this Rust SDK.
/// It is used to create, delete, and manage your indexes and collections.
/// This function constructs a `PineconeClient` struct by attempting to read in environment variables for the required parameters.
///
/// ### Return
/// * `Result<PineconeClient, PineconeError>`
///
/// ### Configuration with environment variables
/// If arguments are not provided, the SDK will attempt to read the following environment variables:
/// - `PINECONE_API_KEY`: The API key used for authentication. If not passed as an argument, it will be read from the environment variable.
/// - `PINECONE_CONTROLLER_HOST`: The Pinecone controller host. Default is `https://api.pinecone.io`.
/// - `PINECONE_ADDITIONAL_HEADERS`: Additional headers to be included in all requests. Expects JSON.
///
/// ### Example
/// ```no_run
/// use pinecone_sdk::pinecone::PineconeClient;
/// 
/// // Create a Pinecone client with the API key and controller host read from environment variables.
/// let pinecone: PineconeClient = pinecone_sdk::pinecone::default_client().expect("Failed to create Pinecone instance");
/// ```
pub fn default_client() -> Result<PineconeClient, PineconeError> {
    PineconeClientConfig::default().client()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    fn empty_headers_with_api_version() -> HashMap<String, String> {
        let mut headers = HashMap::new();
        add_api_version_header(&mut headers);
        headers
    }

    #[tokio::test]
    async fn test_arg_api_key() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key";
        let mock_controller_host = "mock-arg-controller-host";

        let config = PineconeClientConfig {
            api_key: Some(mock_api_key.to_string()),
            control_plane_host: Some(mock_controller_host.to_string()),
            additional_headers: Some(HashMap::new()),
            source_tag: None,
        };

        let pinecone = config
            .client()
            .expect("Expected to successfully create Pinecone instance");

        assert_eq!(pinecone.api_key, mock_api_key);
        assert_eq!(pinecone.controller_url, mock_controller_host);
        assert_eq!(
            pinecone.additional_headers,
            empty_headers_with_api_version()
        );
        assert_eq!(pinecone.source_tag, None);
        assert_eq!(
            pinecone.user_agent,
            Some("lang=rust; pinecone-rust-client=0.1.0".to_string())
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_env_api_key() -> Result<(), PineconeError> {
        let mock_api_key = "mock-env-api-key";
        let mock_controller_host = "mock-arg-controller-host";

        temp_env::with_var("PINECONE_API_KEY", Some(mock_api_key), || {
            let config = PineconeClientConfig {
                control_plane_host: Some(mock_controller_host.to_string()),
                additional_headers: Some(HashMap::new()),
                ..Default::default()
            };
            let pinecone = config
                .client()
                .expect("Expected to successfully create Pinecone instance");

            assert_eq!(pinecone.api_key, mock_api_key);
            assert_eq!(pinecone.controller_url, mock_controller_host);
            assert_eq!(
                pinecone.additional_headers,
                empty_headers_with_api_version()
            );
            assert_eq!(pinecone.source_tag, None);
            assert_eq!(
                pinecone.user_agent,
                Some("lang=rust; pinecone-rust-client=0.1.0".to_string())
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_no_api_key() -> Result<(), PineconeError> {
        let mock_controller_host = "mock-arg-controller-host";

        temp_env::with_var_unset("PINECONE_API_KEY", || {
            let config = PineconeClientConfig {
                control_plane_host: Some(mock_controller_host.to_string()),
                additional_headers: Some(HashMap::new()),
                ..Default::default()
            };
            let pinecone = config
                .client()
                .expect_err("Expected to fail creating Pinecone instance due to missing API key");

            assert!(matches!(pinecone, PineconeError::APIKeyMissingError { .. }));
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_arg_host() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key";
        let mock_controller_host = "mock-arg-controller-host";
        let config = PineconeClientConfig {
            api_key: Some(mock_api_key.to_string()),
            control_plane_host: Some(mock_controller_host.to_string()),
            additional_headers: Some(HashMap::new()),
            source_tag: None,
        };
        let pinecone = config
            .client()
            .expect("Expected to successfully create Pinecone instance");

        assert_eq!(pinecone.controller_url, mock_controller_host);

        Ok(())
    }

    #[tokio::test]
    async fn test_env_host() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key";
        let mock_controller_host = "mock-env-controller-host";

        temp_env::with_var(
            "PINECONE_CONTROLLER_HOST",
            Some(mock_controller_host),
            || {
                let config = PineconeClientConfig {
                    api_key: Some(mock_api_key.to_string()),
                    additional_headers: Some(HashMap::new()),
                    ..Default::default()
                };

                let pinecone = config
                    .client()
                    .expect("Expected to successfully create Pinecone instance with env host");

                assert_eq!(pinecone.controller_url, mock_controller_host);
            },
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_default_host() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key";

        temp_env::with_var_unset("PINECONE_CONTROLLER_HOST", || {
            let config = PineconeClientConfig {
                api_key: Some(mock_api_key.to_string()),
                additional_headers: Some(HashMap::new()),
                ..Default::default()
            };

            let pinecone = config.client().expect(
                "Expected to successfully create Pinecone instance with default controller host",
            );

            assert_eq!(
                pinecone.controller_url,
                "https://api.pinecone.io".to_string()
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_arg_headers() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key";
        let mock_controller_host = "mock-arg-controller-host";
        let mock_headers = HashMap::from([
            ("argheader1".to_string(), "value1".to_string()),
            ("argheader2".to_string(), "value2".to_string()),
        ]);

        let config = PineconeClientConfig {
            api_key: Some(mock_api_key.to_string()),
            control_plane_host: Some(mock_controller_host.to_string()),
            additional_headers: Some(mock_headers.clone()),
            source_tag: None,
        };
        let pinecone = config
            .client()
            .expect("Expected to successfully create Pinecone instance");

        let expected_headers = {
            let mut headers = mock_headers.clone();
            add_api_version_header(&mut headers);
            headers
        };

        assert_eq!(pinecone.additional_headers, expected_headers);

        Ok(())
    }

    #[tokio::test]
    async fn test_env_headers() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key";
        let mock_controller_host = "mock-arg-controller-host";
        let mock_headers = HashMap::from([
            ("envheader1".to_string(), "value1".to_string()),
            ("envheader2".to_string(), "value2".to_string()),
        ]);

        temp_env::with_var(
            "PINECONE_ADDITIONAL_HEADERS",
            Some(serde_json::to_string(&mock_headers).unwrap().as_str()),
            || {
                let config = PineconeClientConfig {
                    api_key: Some(mock_api_key.to_string()),
                    control_plane_host: Some(mock_controller_host.to_string()),
                    additional_headers: None,
                    source_tag: None,
                };

                let pinecone = config
                    .client()
                    .expect("Expected to successfully create Pinecone instance with env headers");

                let expected_headers = {
                    let mut headers = mock_headers.clone();
                    add_api_version_header(&mut headers);
                    headers
                };

                assert_eq!(pinecone.additional_headers, expected_headers);
            },
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_env_headers() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key";
        let mock_controller_host = "mock-arg-controller-host";

        temp_env::with_var("PINECONE_ADDITIONAL_HEADERS", Some("invalid-json"), || {
            let config = PineconeClientConfig {
                api_key: Some(mock_api_key.to_string()),
                control_plane_host: Some(mock_controller_host.to_string()),
                additional_headers: None,
                source_tag: None,
            };
            let pinecone = config
                .client()
                .expect_err("Expected to fail creating Pinecone instance due to invalid headers");

            assert!(matches!(
                pinecone,
                PineconeError::InvalidHeadersError { .. }
            ));
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_default_headers() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key";
        let mock_controller_host = "mock-arg-controller-host";

        temp_env::with_var_unset("PINECONE_ADDITIONAL_HEADERS", || {
            let config = PineconeClientConfig {
                api_key: Some(mock_api_key.to_string()),
                control_plane_host: Some(mock_controller_host.to_string()),
                additional_headers: None,
                source_tag: None,
            };

            let pinecone = config
                .client()
                .expect("Expected to successfully create Pinecone instance");

            assert_eq!(
                pinecone.additional_headers,
                empty_headers_with_api_version()
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_headers_no_api_version() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key";
        let mock_controller_host = "mock-arg-controller-host";

        temp_env::with_var_unset("PINECONE_ADDITIONAL_HEADERS", || {
            let headers = HashMap::from([
                ("HEADER1".to_string(), "value1".to_string()),
                ("HEADER2".to_string(), "value2".to_string()),
            ]);

            let config = PineconeClientConfig {
                api_key: Some(mock_api_key.to_string()),
                control_plane_host: Some(mock_controller_host.to_string()),
                additional_headers: Some(headers.clone()),
                source_tag: None,
            };

            let pinecone = config
                .client()
                .expect("Expected to successfully create Pinecone instance");

            // expect headers, except with the added API version header
            let mut expected_headers = headers.clone();
            expected_headers.insert(
                PINECONE_API_VERSION_KEY.to_string(),
                API_VERSION.to_string(),
            );

            assert_eq!(pinecone.additional_headers, expected_headers);
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_headers_api_version() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key";
        let mock_controller_host = "mock-arg-controller-host";

        temp_env::with_var_unset("PINECONE_ADDITIONAL_HEADERS", || {
            let headers = HashMap::from([
                ("HEADER1".to_string(), "value1".to_string()),
                ("HEADER2".to_string(), "value2".to_string()),
                (
                    PINECONE_API_VERSION_KEY.to_string(),
                    "mock-api-version".to_string(),
                ),
            ]);

            let config = PineconeClientConfig {
                api_key: Some(mock_api_key.to_string()),
                control_plane_host: Some(mock_controller_host.to_string()),
                additional_headers: Some(headers.clone()),
                source_tag: None,
            };

            let pinecone = config
                .client()
                .expect("Expected to successfully create Pinecone instance");

            assert_eq!(pinecone.additional_headers, headers);
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_headers_api_version_different_casing() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key";
        let mock_controller_host = "mock-arg-controller-host";

        temp_env::with_var_unset("PINECONE_ADDITIONAL_HEADERS", || {
            let headers = HashMap::from([
                ("HEADER1".to_string(), "value1".to_string()),
                ("HEADER2".to_string(), "value2".to_string()),
                (
                    "X-pineCONE-api-version".to_string(),
                    "mock-api-version".to_string(),
                ),
            ]);

            let config = PineconeClientConfig {
                api_key: Some(mock_api_key.to_string()),
                control_plane_host: Some(mock_controller_host.to_string()),
                additional_headers: Some(headers.clone()),
                source_tag: None,
            };

            let pinecone = config
                .client()
                .expect("Expected to successfully create Pinecone instance");

            assert_eq!(pinecone.additional_headers, headers);
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_arg_overrides_env() -> Result<(), PineconeError> {
        let mock_arg_api_key = "mock-arg-api-key";
        let mock_arg_controller_host = "mock-arg-controller-host";
        let mock_arg_headers = HashMap::from([
            ("argheader1".to_string(), "value1".to_string()),
            ("argheader2".to_string(), "value2".to_string()),
        ]);
        let mock_env_api_key = "mock-env-api-key";
        let mock_env_controller_host = "mock-env-controller-host";
        let mock_env_headers = HashMap::from([
            ("envheader1".to_string(), "value1".to_string()),
            ("envheader2".to_string(), "value2".to_string()),
        ]);

        temp_env::with_vars(
            [
                ("PINECONE_API_KEY", Some(mock_env_api_key)),
                ("PINECONE_CONTROLLER_HOST", Some(mock_env_controller_host)),
                (
                    "PINECONE_ADDITIONAL_HEADERS",
                    Some(serde_json::to_string(&mock_env_headers).unwrap().as_str()),
                ),
            ],
            || {
                let config = PineconeClientConfig {
                    api_key: Some(mock_arg_api_key.to_string()),
                    control_plane_host: Some(mock_arg_controller_host.to_string()),
                    additional_headers: Some(mock_arg_headers.clone()),
                    source_tag: None,
                };

                let pinecone = config
                    .client()
                    .expect("Expected to successfully create Pinecone instance");

                let expected_headers = {
                    let mut headers = mock_arg_headers.clone();
                    add_api_version_header(&mut headers);
                    headers
                };

                assert_eq!(pinecone.api_key, mock_arg_api_key);
                assert_eq!(pinecone.controller_url, mock_arg_controller_host);
                assert_eq!(pinecone.additional_headers, expected_headers);
            },
        );

        Ok(())
    }
}
