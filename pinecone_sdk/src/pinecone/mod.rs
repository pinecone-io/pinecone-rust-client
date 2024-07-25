use crate::utils::errors::PineconeError;
use crate::utils::user_agent::get_user_agent;
use openapi::apis::configuration::ApiKey;
use openapi::apis::configuration::Configuration;
use reqwest::header::HeaderMap;
use serde_json;
use std::collections::HashMap;
use std::str::FromStr;

/// The `PINECONE_API_VERSION_KEY` is the key for the Pinecone API version header.
pub const PINECONE_API_VERSION_KEY: &str = "x-pinecone-api-version";

/// Control plane module.
pub mod control;

/// Data plane module.
pub mod data;

/// The `PineconeClient` struct is the main entry point for interacting with Pinecone via this Rust SDK.
#[derive(Debug, Clone)]
pub struct PineconeClient {
    api_key: String,
    controller_url: String,
    additional_headers: HashMap<String, String>,
    source_tag: Option<String>,
    user_agent: Option<String>,
    openapi_config: Configuration,
}

impl PineconeClient {
    /// The `PineconeClient` struct is the main entry point for interacting with Pinecone via this Rust SDK.
    /// It is used to create, delete, and manage your indexes and collections.
    ///
    /// ### Arguments
    /// * `api_key: Option<&str>` - The API key used for authentication.
    /// * `control_plane_host: Option<&str>` - The Pinecone controller host. Default is `https://api.pinecone.io`.
    /// * `additional_headers: Option<HashMap<String, String>>` - Additional headers to be included in all requests. Expects a HashMap.
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
    /// use pinecone_sdk::pinecone::PineconeClient;
    ///
    /// // Create a Pinecone client with the API key and controller host.
    /// let pinecone = PineconeClient::new(Some("INSERT_API_KEY"), Some("INSERT_CONTROLLER_HOST"), None, None);
    /// ```
    pub fn new(
        api_key: Option<&str>,
        control_plane_host: Option<&str>,
        additional_headers: Option<HashMap<String, String>>,
        source_tag: Option<&str>,
    ) -> Result<Self, PineconeError> {
        // get api key
        let api_key = match api_key {
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

        let env_controller = &std::env::var("PINECONE_CONTROLLER_HOST")
            .unwrap_or("https://api.pinecone.io".to_string());
        let controller_host = control_plane_host.unwrap_or(env_controller);

        // get additional headers
        let additional_headers =
            additional_headers.unwrap_or(match std::env::var("PINECONE_ADDITIONAL_HEADERS") {
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

        // get user agent
        let user_agent = get_user_agent(source_tag);

        // create reqwest headers
        let headers: Result<HeaderMap, PineconeError> = additional_headers
            .iter()
            .map(|(name, val)| {
                (
                    reqwest::header::HeaderName::from_str(name).map_err(|e| {
                        PineconeError::InvalidHeadersError {
                            message: e.to_string(),
                        }
                    }),
                    reqwest::header::HeaderValue::from_str(val.as_ref()).map_err(|e| {
                        PineconeError::InvalidHeadersError {
                            message: e.to_string(),
                        }
                    }),
                )
            })
            .map(
                |(name_result, val_result)| match (name_result, val_result) {
                    (Ok(name), Ok(val)) => Ok((name, val)),
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                },
            )
            .collect();

        let mut headers = headers?;

        // add X-Pinecone-Api-Version header if not present
        if !headers.contains_key(PINECONE_API_VERSION_KEY) {
            headers.insert(
                reqwest::header::HeaderName::from_static(PINECONE_API_VERSION_KEY),
                reqwest::header::HeaderValue::from_static(crate::version::API_VERSION),
            );
        }

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
        Ok(PineconeClient {
            api_key,
            controller_url: controller_host.to_string(),
            additional_headers,
            source_tag: source_tag.map(|s| s.to_string()),
            user_agent: Some(user_agent),
            openapi_config,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_arg_api_key() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key";
        let mock_controller_host = "mock-arg-controller-host";

        let pinecone = PineconeClient::new(
            Some(mock_api_key),
            Some(mock_controller_host),
            Some(HashMap::new()),
            None,
        )
        .expect("Expected to successfully create Pinecone instance");

        assert_eq!(pinecone.api_key, mock_api_key);
        assert_eq!(pinecone.controller_url, mock_controller_host);
        assert_eq!(pinecone.additional_headers, HashMap::new());
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
            let pinecone =
                PineconeClient::new(None, Some(mock_controller_host), Some(HashMap::new()), None)
                    .expect("Expected to successfully create Pinecone instance");

            assert_eq!(pinecone.api_key, mock_api_key);
            assert_eq!(pinecone.controller_url, mock_controller_host);
            assert_eq!(pinecone.additional_headers, HashMap::new());
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
            let pinecone =
                PineconeClient::new(None, Some(mock_controller_host), Some(HashMap::new()), None)
                    .expect_err(
                        "Expected to fail creating Pinecone instance due to missing API key",
                    );

            assert!(matches!(pinecone, PineconeError::APIKeyMissingError { .. }));
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_arg_host() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key";
        let mock_controller_host = "mock-arg-controller-host";
        let pinecone = PineconeClient::new(
            Some(mock_api_key),
            Some(mock_controller_host),
            Some(HashMap::new()),
            None,
        )
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
                let pinecone =
                    PineconeClient::new(Some(mock_api_key), None, Some(HashMap::new()), None)
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
            let pinecone = PineconeClient::new(
                Some(mock_api_key),
                None,
                Some(HashMap::new()),
                None,
            )
            .expect(
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

        let pinecone = PineconeClient::new(
            Some(mock_api_key),
            Some(mock_controller_host),
            Some(mock_headers.clone()),
            None,
        )
        .expect("Expected to successfully create Pinecone instance");

        assert_eq!(pinecone.additional_headers, mock_headers);

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
                let pinecone =
                    PineconeClient::new(Some(mock_api_key), Some(mock_controller_host), None, None)
                        .expect(
                            "Expected to successfully create Pinecone instance with env headers",
                        );

                assert_eq!(pinecone.additional_headers, mock_headers);
            },
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_env_headers() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key";
        let mock_controller_host = "mock-arg-controller-host";

        temp_env::with_var("PINECONE_ADDITIONAL_HEADERS", Some("invalid-json"), || {
            let pinecone =
                PineconeClient::new(Some(mock_api_key), Some(mock_controller_host), None, None)
                    .expect_err(
                        "Expected to fail creating Pinecone instance due to invalid headers",
                    );

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
            let pinecone = PineconeClient::new(
                Some(mock_api_key),
                Some(mock_controller_host),
                Some(HashMap::new()),
                None,
            )
            .expect("Expected to successfully create Pinecone instance");

            assert_eq!(pinecone.additional_headers, HashMap::new());
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
                let pinecone = PineconeClient::new(
                    Some(mock_arg_api_key),
                    Some(mock_arg_controller_host),
                    Some(mock_arg_headers.clone()),
                    None,
                )
                .expect("Expected to successfully create Pinecone instance");

                assert_eq!(pinecone.api_key, mock_arg_api_key);
                assert_eq!(pinecone.controller_url, mock_arg_controller_host);
                assert_eq!(pinecone.additional_headers, mock_arg_headers.clone());
            },
        );

        Ok(())
    }
}
