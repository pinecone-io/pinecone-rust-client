use pinecone_sdk::models::EmbedRequestParameters;
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::utils::errors::PineconeError;

#[tokio::test]
async fn test_embed() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(Default::default()).expect("Failed to create client");

    let parameters = EmbedRequestParameters {
        input_type: Some("query".to_string()),
        truncate: Some("END".to_string()),
    };

    let response = pinecone
        .embed(
            "multilingual-e5-large",
            Some(parameters),
            &vec!["Hello, world!"],
        )
        .await
        .expect("Failed to embed");

    assert_eq!(response.model, "multilingual-e5-large");
    assert_eq!(response.data.len(), 1);

    Ok(())
}

#[tokio::test]
async fn test_embed_invalid_model() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(Default::default()).expect("Failed to create client");

    let _ = pinecone
        .embed("invalid-model", None, &vec!["Hello, world!"])
        .await
        .expect_err("Expected to fail embedding with invalid model");

    Ok(())
}

#[tokio::test]
async fn test_embed_invalid_parameters() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(Default::default()).expect("Failed to create client");

    let parameters = EmbedRequestParameters {
        input_type: Some("bad-parameter".to_string()),
        truncate: Some("bad-parameter".to_string()),
    };

    let _ = pinecone
        .embed(
            "multilingual-e5-large",
            Some(parameters),
            &vec!["Hello, world!"],
        )
        .await
        .expect_err("Expected to fail embedding with invalid model parameters");

    Ok(())
}
