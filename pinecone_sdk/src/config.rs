use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub controller_url: String,
    pub additional_headers: HashMap<String, String>,
    pub source_tag: Option<String>,
}

impl Config {
    pub fn new(api_key: String, source_tag: Option<String>) -> Self {
        Config {
            api_key,
            controller_url: "https://api.pinecone.io".to_string(),
            additional_headers: HashMap::new(),
            source_tag,
        }
    }
}
