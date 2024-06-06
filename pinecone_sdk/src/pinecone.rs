use crate::config::Config;
use crate::utils::user_agent::get_user_agent;
use openapi::apis::configuration::ApiKey;
use openapi::apis::configuration::Configuration;

#[derive(Debug, Clone)]
pub struct Pinecone {
    config: Config,
    openapi_config: Configuration,
}

impl Pinecone {
    pub fn new(api_key: String, control_plane_host: Option<String>, source_tag: Option<String>) -> Self {
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

        Pinecone {
            config,
            openapi_config,
        }
    }

    pub fn openapi_config(&self) -> &Configuration {
        &self.openapi_config
    }
}
