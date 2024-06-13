use crate::config::Config;
use crate::utils::errors::PineconeError;
use crate::utils::user_agent::get_user_agent;
use openapi::apis::configuration::ApiKey;
use openapi::apis::configuration::Configuration;
use serde_json;
use std::collections::HashMap;

/// The `Pinecone` struct is the main entry point for interacting with Pinecone via this Rust SDK.
#[derive(Debug, Clone)]
pub struct PineconeClient {
    /// Configuration for the Pinecone SDK struct.
    config: Config,

    /// OpenAPI configuration object.
    openapi_config: Configuration,
}

impl PineconeClient {
    /// The `Pinecone` struct is the main entry point for interacting with Pinecone via this Rust SDK.
    /// It is used to create, delete, and manage your indexes and collections.
    ///
    /// ### Configuration with environment variables
    ///
    /// If arguments are not provided, the SDK will attempt to read the following environment variables:
    /// - `PINECONE_API_KEY`: The API key used for authentication. If not passed as an argument, it will be read from the environment variable.
    /// - `PINECONE_CONTROLLER_HOST`: The Pinecone controller host. Default is `https://api.pinecone.io`.
    /// - `PINECONE_ADDITIONAL_HEADERS`: Additional headers to be included in all requests. Expects JSON.
    ///
    /// ### Example
    ///
    /// ```
    /// use pinecone_sdk::pinecone::PineconeClient;
    ///
    /// // Create a Pinecone client with the API key and controller host.
    /// let pinecone = PineconeClient::new(Some("INSERT_API_KEY".to_string()), Some("INSERT_CONTROLLER_HOST".to_string()), None, None);
    /// ```
    pub fn new(
        api_key: Option<String>,
        control_plane_host: Option<String>,
        additional_headers: Option<HashMap<String, String>>,
        source_tag: Option<String>,
    ) -> Result<Self, PineconeError> {
        // get api key
        let api_key = match api_key {
            Some(key) => key,
            None => match std::env::var("PINECONE_API_KEY") {
                Ok(key) => key,
                Err(_) => {
                    return Err(PineconeError::APIKeyMissingError);
                }
            },
        };

        let controller_host = control_plane_host.unwrap_or(
            std::env::var("PINECONE_CONTROLLER_HOST")
                .unwrap_or("https://api.pinecone.io".to_string()),
        );

        let additional_headers = match additional_headers {
            Some(headers) => headers,
            None => match std::env::var("PINECONE_ADDITIONAL_HEADERS") {
                Ok(headers) => match serde_json::from_str(&headers) {
                    Ok(headers) => headers,
                    Err(json_error) => {
                        return Err(PineconeError::InvalidHeadersError { json_error });
                    }
                },
                Err(_) => HashMap::new(),
            },
        };

        let config = Config {
            api_key: api_key.clone(),
            controller_url: controller_host.clone(),
            additional_headers,
            source_tag,
        };

        let user_agent = get_user_agent(&config);

        let openapi_config = Configuration {
            base_path: controller_host,
            user_agent: Some(user_agent),
            api_key: Some(ApiKey {
                prefix: None,
                key: api_key,
            }),
            ..Default::default()
        };

        Ok(PineconeClient {
            config,
            openapi_config,
        })
    }

    /// Constructs a PineconeBuilder instance
    pub fn builder() -> PineconeBuilder {
        PineconeBuilder::new()
    }

    /// Returns the OpenAPI configuration object.
    pub fn openapi_config(&self) -> &Configuration {
        &self.openapi_config
    }
}

/// The `PineconeBuilder` struct is used to construct a `Pinecone` instance with a builder pattern.
pub struct PineconeBuilder {
    api_key: Option<String>,
    control_plane_host: Option<String>,
    additional_headers: Option<HashMap<String, String>>,
    source_tag: Option<String>,
}

impl PineconeBuilder {
    /// Constructs a new PineconeBuilder instance.
    pub fn new() -> PineconeBuilder {
        PineconeBuilder {
            api_key: None,
            control_plane_host: None,
            additional_headers: None,
            source_tag: None,
        }
    }

    /// Sets the API key for the Pinecone instance.
    pub fn api_key(mut self, api_key: &str) -> PineconeBuilder {
        self.api_key = Some(api_key.to_string());
        self
    }

    /// Sets the control plane host for the Pinecone instance.
    pub fn control_plane_host(mut self, control_plane_host: &str) -> PineconeBuilder {
        self.control_plane_host = Some(control_plane_host.to_string());
        self
    }

    /// Sets additional headers for the Pinecone instance.
    pub fn additional_headers(
        mut self,
        additional_headers: HashMap<String, String>,
    ) -> PineconeBuilder {
        self.additional_headers = Some(additional_headers);
        self
    }

    /// Sets the source tag for the Pinecone instance.
    pub fn source_tag(mut self, source_tag: &str) -> PineconeBuilder {
        self.source_tag = Some(source_tag.to_string());
        self
    }

    /// Constructs Pinecone instance from PineconeBuilder fields.
    pub fn build(self) -> Result<PineconeClient, PineconeError> {
        PineconeClient::new(
            self.api_key,
            self.control_plane_host,
            self.additional_headers,
            self.source_tag,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_arg_api_key() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key".to_string();
        let mock_controller_host = "mock-arg-controller-host".to_string();

        let pinecone = PineconeClient::new(
            Some(mock_api_key.clone()),
            Some(mock_controller_host.clone()),
            Some(HashMap::new()),
            None,
        )
        .expect("Expected to successfully create Pinecone instance");

        assert_eq!(pinecone.config.api_key, mock_api_key.clone());

        Ok(())
    }

    #[tokio::test]
    async fn test_env_api_key() -> Result<(), PineconeError> {
        let mock_api_key = "mock-env-api-key".to_string();
        let mock_controller_host = "mock-arg-controller-host".to_string();

        temp_env::with_var("PINECONE_API_KEY", Some(mock_api_key.as_str()), || {
            let pinecone = PineconeClient::new(
                None,
                Some(mock_controller_host.clone()),
                Some(HashMap::new()),
                None,
            )
            .expect("Expected to successfully create Pinecone instance");

            assert_eq!(pinecone.config.api_key, mock_api_key.clone());
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_no_api_key() -> Result<(), PineconeError> {
        let mock_controller_host = "mock-arg-controller-host".to_string();

        temp_env::with_var_unset("PINECONE_API_KEY", || {
            let pinecone = PineconeClient::new(
                None,
                Some(mock_controller_host.clone()),
                Some(HashMap::new()),
                None,
            )
            .expect_err("Expected to fail creating Pinecone instance due to missing API key");

            assert!(matches!(pinecone, PineconeError::APIKeyMissingError));
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_arg_host() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key".to_string();
        let mock_controller_host = "mock-arg-controller-host".to_string();
        let pinecone = PineconeClient::new(
            Some(mock_api_key.clone()),
            Some(mock_controller_host.clone()),
            Some(HashMap::new()),
            None,
        )
        .expect("Expected to successfully create Pinecone instance");

        assert_eq!(pinecone.config.controller_url, mock_controller_host.clone());

        Ok(())
    }

    #[tokio::test]
    async fn test_env_host() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key".to_string();
        let mock_controller_host = "mock-env-controller-host".to_string();

        temp_env::with_var(
            "PINECONE_CONTROLLER_HOST",
            Some(mock_controller_host.as_str()),
            || {
                let pinecone =
                    PineconeClient::new(Some(mock_api_key.clone()), None, Some(HashMap::new()), None)
                        .expect("Expected to successfully create Pinecone instance with env host");

                assert_eq!(pinecone.config.controller_url, mock_controller_host.clone());
            },
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_default_host() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key".to_string();

        temp_env::with_var_unset("PINECONE_CONTROLLER_HOST", || {
            let pinecone = PineconeClient::new(
                Some(mock_api_key.clone()),
                None,
                Some(HashMap::new()),
                None,
            )
            .expect(
                "Expected to successfully create Pinecone instance with default controller host",
            );

            assert_eq!(
                pinecone.config.controller_url,
                "https://api.pinecone.io".to_string()
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_arg_headers() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key".to_string();
        let mock_controller_host = "mock-arg-controller-host".to_string();
        let mock_headers = HashMap::from([
            ("argheader1".to_string(), "value1".to_string()),
            ("argheader2".to_string(), "value2".to_string()),
        ]);

        let pinecone = PineconeClient::new(
            Some(mock_api_key.clone()),
            Some(mock_controller_host.clone()),
            Some(mock_headers.clone()),
            None,
        )
        .expect("Expected to successfully create Pinecone instance");

        assert_eq!(pinecone.config.additional_headers, mock_headers.clone());

        Ok(())
    }

    #[tokio::test]
    async fn test_env_headers() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key".to_string();
        let mock_controller_host = "mock-arg-controller-host".to_string();
        let mock_headers = HashMap::from([
            ("envheader1".to_string(), "value1".to_string()),
            ("envheader2".to_string(), "value2".to_string()),
        ]);

        temp_env::with_var(
            "PINECONE_ADDITIONAL_HEADERS",
            Some(serde_json::to_string(&mock_headers).unwrap().as_str()),
            || {
                let pinecone = PineconeClient::new(
                    Some(mock_api_key.clone()),
                    Some(mock_controller_host.clone()),
                    None,
                    None,
                )
                .expect("Expected to successfully create Pinecone instance with env headers");

                assert_eq!(pinecone.config.additional_headers, mock_headers.clone());
            },
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_env_headers() -> Result<(), PineconeError> {
        let mock_api_key = "mock-arg-api-key".to_string();
        let mock_controller_host = "mock-arg-controller-host".to_string();

        temp_env::with_var("PINECONE_ADDITIONAL_HEADERS", Some("invalid-json"), || {
            let pinecone = PineconeClient::new(
                Some(mock_api_key.clone()),
                Some(mock_controller_host.clone()),
                None,
                None,
            )
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
        let mock_api_key = "mock-arg-api-key".to_string();
        let mock_controller_host = "mock-arg-controller-host".to_string();

        temp_env::with_var_unset("PINECONE_ADDITIONAL_HEADERS", || {
            let pinecone = PineconeClient::new(
                Some(mock_api_key.clone()),
                Some(mock_controller_host.clone()),
                Some(HashMap::new()),
                None,
            )
            .expect("Expected to successfully create Pinecone instance");

            assert_eq!(pinecone.config.additional_headers, HashMap::new());
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_arg_overrides_env() -> Result<(), PineconeError> {
        let mock_arg_api_key = "mock-arg-api-key".to_string();
        let mock_arg_controller_host = "mock-arg-controller-host".to_string();
        let mock_arg_headers = HashMap::from([
            ("argheader1".to_string(), "value1".to_string()),
            ("argheader2".to_string(), "value2".to_string()),
        ]);
        let mock_env_api_key = "mock-env-api-key".to_string();
        let mock_env_controller_host = "mock-env-controller-host".to_string();
        let mock_env_headers = HashMap::from([
            ("envheader1".to_string(), "value1".to_string()),
            ("envheader2".to_string(), "value2".to_string()),
        ]);

        temp_env::with_vars(
            [
                ("PINECONE_API_KEY", Some(mock_env_api_key.as_str())),
                (
                    "PINECONE_CONTROLLER_HOST",
                    Some(mock_env_controller_host.as_str()),
                ),
                (
                    "PINECONE_ADDITIONAL_HEADERS",
                    Some(serde_json::to_string(&mock_env_headers).unwrap().as_str()),
                ),
            ],
            || {
                let pinecone = PineconeClient::new(
                    Some(mock_arg_api_key.clone()),
                    Some(mock_arg_controller_host.clone()),
                    Some(mock_arg_headers.clone()),
                    None,
                )
                .expect("Expected to successfully create Pinecone instance");

                assert_eq!(pinecone.config.api_key, mock_arg_api_key.clone());
                assert_eq!(
                    pinecone.config.controller_url,
                    mock_arg_controller_host.clone()
                );
                assert_eq!(pinecone.config.additional_headers, mock_arg_headers.clone());
            },
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_builder() {
        let pinecone = PineconeClient::builder().api_key("mock-api-key").build();

        assert_eq!(pinecone.unwrap().config.api_key, "mock-api-key");
    }

    #[tokio::test]
    async fn test_builder_all_params() {
        let pinecone = PineconeClient::builder()
            .api_key("mock-api-key")
            .additional_headers(HashMap::from([(
                "header1".to_string(),
                "value1".to_string(),
            )]))
            .control_plane_host("mock-controller-host")
            .source_tag("mock-source-tag")
            .build()
            .unwrap();

        assert_eq!(pinecone.config.api_key, "mock-api-key");
        assert_eq!(
            pinecone.config.additional_headers,
            HashMap::from([("header1".to_string(), "value1".to_string())])
        );
        assert_eq!(pinecone.config.controller_url, "mock-controller-host");
        assert_eq!(
            pinecone.config.source_tag,
            Some("mock-source-tag".to_string())
        );
    }
}
