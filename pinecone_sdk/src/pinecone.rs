use crate::config::Config;
use openapi::apis::configuration::Configuration;
use openapi::apis::configuration::ApiKey;

#[derive(Debug, Clone)]
pub struct Pinecone {
    config: Config,
    openapi_config: Configuration
}

impl Pinecone {
    pub fn new(api_key: String) -> Self {
        let config = Config::new(api_key.clone());
        
        let openapi_config = Configuration {
            base_path: "https://api.pinecone.io".to_string(),
            user_agent: Some("pinecone-rust-client".to_string()),
            api_key: Some(ApiKey {
                prefix: None,
                key: api_key,
            }),
            ..Default::default()
        };

        Pinecone { config, openapi_config }
    }

    pub fn openapi_config(&self) -> &Configuration {
        &self.openapi_config
    }
}
