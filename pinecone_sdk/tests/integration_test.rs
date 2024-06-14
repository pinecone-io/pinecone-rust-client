use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::utils::errors::PineconeError;

#[tokio::test]
async fn test_describe_index() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    let _ = pinecone
        .describe_index("describe-index")
        .await
        .expect("Failed to describe index");

    Ok(())
}

#[tokio::test]
async fn test_describe_index_fail() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    let _ = pinecone
        .describe_index("non-existent-index")
        .await
        .expect_err("Expected to fail describing index");

    Ok(())
}

#[tokio::test]
async fn test_list_index() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    let _ = pinecone
        .list_indexes()
        .await
        .expect("Failed to list indexes");

    Ok(())
}
