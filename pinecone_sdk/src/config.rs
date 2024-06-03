#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub controller_url: String,
}

impl Config {
    pub fn new(api_key: String) -> Self {
        Config {
            api_key,
            controller_url: "https://api.pinecone.io".to_string(),
        }
    }
}
