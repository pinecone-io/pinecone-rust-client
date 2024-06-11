use pinecone_sdk::pinecone::Pinecone;

#[tokio::test]
async fn test_list_serverless_index_env() {
    let pinecone = Pinecone::new(None, None, None, None).unwrap();
    let list_response = pinecone.list_indexes().await;

    assert!(list_response.is_ok());
}