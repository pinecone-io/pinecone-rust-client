use regex::Regex;
use crate::config::Config;

// Normalizes the source tag
fn build_source_tag(source_tag: &String) -> String {
    // 1. Lowercase
    // 2. Limit charset to [a-z0-9_ ]
    // 3. Trim left/right empty space
    // 4. Condence multiple spaces to one, and replace with underscore

    let re = Regex::new(r"[^a-z0-9_ ]").unwrap();
    let lowercase_tag = source_tag.to_lowercase();
    let tag = re.replace_all(&lowercase_tag, "");
    return tag.trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("_");
}

// Gets user agent string
pub fn get_user_agent(config: &Config) -> String {
    let mut user_agent = format!("lang=rust/{}", "0.1.0");
    if let Some(source_tag) = &config.source_tag {
        user_agent.push_str(&format!("; source_tag={}", build_source_tag(source_tag)));
    }
    return user_agent;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_build_source_tag() {
        let source_tag = "    Hello   World!! ".to_string();
        assert_eq!(build_source_tag(&source_tag), "hello_world");
    }

    #[tokio::test]
    async fn test_no_source_tag() {
        let config = Config {
            api_key: "api".to_string(),
            controller_url: "https://api.pinecone.io".to_string(),
            source_tag: None,
        };
        assert_eq!(get_user_agent(&config), "lang=rust/0.1.0");
    }

    #[tokio::test]
    async fn test_with_source_tag() {
        let config = Config {
            api_key: "api".to_string(),
            controller_url: "https://api.pinecone.io".to_string(),
            source_tag: Some("tag".to_string()),
        };
        assert_eq!(get_user_agent(&config), "lang=rust/0.1.0; source_tag=tag");
    }
}