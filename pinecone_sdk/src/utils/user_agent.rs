use regex::Regex;

/// Normalizes the source tag.
fn build_source_tag(source_tag: &String) -> String {
    // 1. Lowercase
    // 2. Limit charset to [a-z0-9_ ]
    // 3. Trim left/right empty space
    // 4. Condense multiple spaces to one, and replace with underscore

    let re = Regex::new(r"[^a-z0-9_: ]").unwrap();
    let lowercase_tag = source_tag.to_lowercase();
    let tag = re.replace_all(&lowercase_tag, "");
    return tag
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("_");
}

/// Gets the user-agent string.
pub fn get_user_agent(source_tag: Option<&str>) -> String {
    let mut user_agent = format!("lang=rust; pinecone-rust-client={}", "0.1.0");
    if let Some(st) = source_tag {
        user_agent.push_str(&format!(
            "; source_tag={}",
            build_source_tag(&st.to_string())
        ));
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
    async fn test_build_source_tag_special_chars() {
        let source_tag = " Hello   World__:_!@#@#   ".to_string();
        assert_eq!(build_source_tag(&source_tag), "hello_world__:_");
    }

    #[tokio::test]
    async fn test_no_source_tag() {
        assert_eq!(
            get_user_agent(None),
            "lang=rust; pinecone-rust-client=0.1.0"
        );
    }

    #[tokio::test]
    async fn test_with_source_tag() {
        assert_eq!(
            get_user_agent(Some("tag")),
            "lang=rust; pinecone-rust-client=0.1.0; source_tag=tag"
        );
    }
}
