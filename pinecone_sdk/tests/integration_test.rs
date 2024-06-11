use pinecone_sdk::pinecone::Pinecone;
use pinecone_sdk::utils::errors::PineconeError;

#[tokio::test]
async fn test_list_index_env() -> Result<(), PineconeError> {
    let pinecone = Pinecone::new(None, None, None, None).unwrap();
    let _ = pinecone
        .list_indexes()
        .await
        .expect("Failed to list indexes");

    Ok(())
}
