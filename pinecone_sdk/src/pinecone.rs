use crate::config::Config;
use crate::utils::errors::{PineconeError, PineconeErrorKind};
use crate::utils::user_agent::get_user_agent;
use openapi::apis::configuration::ApiKey;
use openapi::apis::configuration::Configuration;

#[derive(Debug, Clone)]
pub struct Pinecone {
    config: Config,
    openapi_config: Configuration,
}

impl Pinecone {
    pub fn new(
        api_key: Option<String>,
        control_plane_host: Option<String>,
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

        let config = Config::new(api_key.clone(), source_tag);

        let user_agent = get_user_agent(&config);

        let openapi_config = Configuration {
            base_path: control_plane_host.unwrap_or("https://api.pinecone.io".to_string()),
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

        let pinecone = Pinecone::new(None, Some(mock_controller_host.clone()), None);

        assert!(pinecone.is_ok());
        assert_eq!(pinecone.unwrap().config.api_key, mock_api_key.clone());
    }

    #[tokio::test]
    #[serial]
    async fn test_no_api_key() {
        let mock_controller_host = "mock-arg-controller-host".to_string();

        env::remove_var("PINECONE_API_KEY");
        assert!(env::var("PINECONE_API_KEY").is_err());

        let pinecone = Pinecone::new(None, Some(mock_controller_host.clone()), None);

        assert!(pinecone.is_err());
    }
}
