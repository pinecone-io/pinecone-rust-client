use std::collections::HashMap;

/// Configuration for the Pinecone SDK struct.
#[derive(Debug, Clone)]
pub struct Config {
    /// The API key for your Pinecone project. You can find this in the [Pinecone console](https://app.pinecone.io).
    pub api_key: String,

    /// Optional configuration field for specifying the controller host.
    pub controller_url: String,

    /// Optional headers to be included in all requests.
    pub additional_headers: HashMap<String, String>,

    /// Optional sourceTag that is applied to the User-Agent header with all requests.
    pub source_tag: Option<String>,
}

impl Config {
    /// Builds a new Config struct.
    pub fn new(api_key: String, source_tag: Option<String>) -> Self {
        Config {
            api_key,
            controller_url: "https://api.pinecone.io".to_string(),
            additional_headers: HashMap::new(),
            source_tag,
        }
    }
}
