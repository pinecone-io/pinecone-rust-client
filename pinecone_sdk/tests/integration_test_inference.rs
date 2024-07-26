use pinecone_sdk::pinecone::inference::EmbedRequestParameters;
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::utils::errors::PineconeError;

#[tokio::test]
async fn test_embed() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

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

    assert_eq!(response.model.unwrap(), "multilingual-e5-large");
    assert_eq!(response.data.unwrap().len(), 1);

    Ok(())
}

#[tokio::test]
async fn test_embed_invalid_model() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let _ = pinecone
        .embed("invalid-model", None, &vec!["Hello, world!"])
        .await
        .expect_err("Expected to fail embedding with invalid model");

    Ok(())
}
