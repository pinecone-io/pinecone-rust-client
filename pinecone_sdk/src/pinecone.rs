use crate::config::Config;
use crate::utils::errors::PineconeError;
use crate::utils::user_agent::get_user_agent;
use openapi::apis::configuration::ApiKey;
use openapi::apis::configuration::Configuration;
use serde_json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Pinecone {
    config: Config,
    openapi_config: Configuration,
}

impl Pinecone {
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

        Ok(Pinecone {
            config,
            openapi_config,
        })
    }

    pub fn openapi_config(&self) -> &Configuration {
        &self.openapi_config
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

        let pinecone = Pinecone::new(
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
            let pinecone = Pinecone::new(
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
            let pinecone = Pinecone::new(
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
        let pinecone = Pinecone::new(
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
                    Pinecone::new(Some(mock_api_key.clone()), None, Some(HashMap::new()), None)
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
            let pinecone = Pinecone::new(
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

        let pinecone = Pinecone::new(
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
                let pinecone = Pinecone::new(
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
            let pinecone = Pinecone::new(
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
            let pinecone = Pinecone::new(
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
                let pinecone = Pinecone::new(
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
}
