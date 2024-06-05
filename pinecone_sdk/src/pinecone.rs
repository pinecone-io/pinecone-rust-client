use crate::config::Config;
use crate::utils::errors::{PineconeError, PineconeErrorKind};
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
                    return Err(PineconeError {
                        kind: PineconeErrorKind::CofigurationError,
                        message: "API key not found. Pass an API key as an argument or set PINECONE_API_KEY in env.".to_string(),
                    });
                }
            },
        };

        let controller_host = control_plane_host.unwrap_or(
            std::env::var("PINECONE_CONTROLLER_HOST")
                .unwrap_or("https://api.pinecone.io".to_string()),
        );

        let additional_headers =
            additional_headers.unwrap_or(match std::env::var("PINECONE_ADDITIONAL_HEADERS") {
                Ok(headers) => serde_json::from_str(&headers).unwrap_or(HashMap::new()),
                Err(_) => HashMap::new(),
            });

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
    use std::env;

    use super::*;
    use serial_test::serial;
    use tokio;

    #[tokio::test]
    async fn test_arg_api_key() {
        let mock_api_key = "mock-arg-api-key".to_string();
        let mock_controller_host = "mock-arg-controller-host".to_string();

        let pinecone = Pinecone::new(
            Some(mock_api_key.clone()),
            Some(mock_controller_host.clone()),
            Some(HashMap::new()),
            None,
        );

        assert!(pinecone.is_ok());
        assert_eq!(pinecone.unwrap().config.api_key, mock_api_key.clone());
    }

    #[tokio::test]
    #[serial]
    async fn test_env_api_key() {
        let mock_api_key = "mock-env-api-key".to_string();
        let mock_controller_host = "mock-arg-controller-host".to_string();

        env::set_var("PINECONE_API_KEY", mock_api_key.clone());
        assert!(env::var("PINECONE_API_KEY").is_ok());
        assert!(env::var("PINECONE_API_KEY").unwrap() == mock_api_key.clone());

        let pinecone = Pinecone::new(
            None,
            Some(mock_controller_host.clone()),
            Some(HashMap::new()),
            None,
        );

        assert!(pinecone.is_ok());
        assert_eq!(pinecone.unwrap().config.api_key, mock_api_key.clone());
    }

    #[tokio::test]
    #[serial]
    async fn test_no_api_key() {
        let mock_controller_host = "mock-arg-controller-host".to_string();

        env::remove_var("PINECONE_API_KEY");
        assert!(env::var("PINECONE_API_KEY").is_err());

        let pinecone = Pinecone::new(
            None,
            Some(mock_controller_host.clone()),
            Some(HashMap::new()),
            None,
        );

        assert!(pinecone.is_err());
    }

    #[tokio::test]
    async fn test_arg_host() {
        let mock_api_key = "mock-arg-api-key".to_string();
        let mock_controller_host = "mock-arg-controller-host".to_string();
        let pinecone = Pinecone::new(
            Some(mock_api_key.clone()),
            Some(mock_controller_host.clone()),
            Some(HashMap::new()),
            None,
        );

        assert!(pinecone.is_ok());
        assert_eq!(
            pinecone.unwrap().config.controller_url,
            mock_controller_host.clone()
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_env_host() {
        let mock_api_key = "mock-arg-api-key".to_string();
        let mock_controller_host = "mock-env-controller-host".to_string();

        env::set_var("PINECONE_CONTROLLER_HOST", mock_controller_host.clone());
        assert!(env::var("PINECONE_CONTROLLER_HOST").is_ok());
        assert!(env::var("PINECONE_CONTROLLER_HOST").unwrap() == mock_controller_host.clone());

        let pinecone = Pinecone::new(Some(mock_api_key.clone()), None, Some(HashMap::new()), None);

        assert!(pinecone.is_ok());
        assert_eq!(
            pinecone.unwrap().config.controller_url,
            mock_controller_host.clone()
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_default_host() {
        let mock_api_key = "mock-arg-api-key".to_string();

        env::remove_var("PINECONE_CONTROLLER_HOST");
        assert!(env::var("PINECONE_CONTROLLER_HOST").is_err());

        let pinecone = Pinecone::new(Some(mock_api_key.clone()), None, Some(HashMap::new()), None);

        assert!(pinecone.is_ok());
        assert_eq!(
            pinecone.unwrap().config.controller_url,
            "https://api.pinecone.io".to_string()
        );
    }

    #[tokio::test]
    async fn test_arg_headers() {
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
        );

        assert!(pinecone.is_ok());
        assert_eq!(
            pinecone.unwrap().config.additional_headers,
            mock_headers.clone()
        );
    }

    #[tokio::test]
    async fn test_env_headers() {
        let mock_api_key = "mock-arg-api-key".to_string();
        let mock_controller_host = "mock-arg-controller-host".to_string();
        let mock_headers = HashMap::from([
            ("envheader1".to_string(), "value1".to_string()),
            ("envheader2".to_string(), "value2".to_string()),
        ]);

        env::set_var(
            "PINECONE_ADDITIONAL_HEADERS",
            serde_json::to_string(&mock_headers).unwrap(),
        );
        assert!(env::var("PINECONE_ADDITIONAL_HEADERS").is_ok());
        assert!(
            env::var("PINECONE_ADDITIONAL_HEADERS").unwrap()
                == serde_json::to_string(&mock_headers).unwrap()
        );

        let pinecone = Pinecone::new(
            Some(mock_api_key.clone()),
            Some(mock_controller_host.clone()),
            None,
            None,
        );

        assert!(pinecone.is_ok());
        assert_eq!(
            pinecone.unwrap().config.additional_headers,
            mock_headers.clone()
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_default_headers() {
        let mock_api_key = "mock-arg-api-key".to_string();
        let mock_controller_host = "mock-arg-controller-host".to_string();

        env::remove_var("PINECONE_CONTROLLER_HOST");
        assert!(env::var("PINECONE_CONTROLLER_HOST").is_err());

        let pinecone = Pinecone::new(
            Some(mock_api_key.clone()),
            Some(mock_controller_host.clone()),
            Some(HashMap::new()),
            None,
        );

        assert!(pinecone.is_ok());
        assert_eq!(pinecone.unwrap().config.additional_headers, HashMap::new());
    }
}
