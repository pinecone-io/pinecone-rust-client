use pinecone_sdk::pinecone::Pinecone;

#[tokio::test]
async fn test_list_serverless_index() {
    let pinecone = Pinecone::new(Some("b41b6453-9756-45aa-8d8d-a51c295d3c78".to_string()), None, None, None).unwrap();
    let list_response = pinecone.list_indexes().await;

    assert!(list_response.is_ok());
}

#[tokio::test]
async fn test_list_serverless_index_env() {
    let pinecone = Pinecone::new(None, None, None, None).unwrap();
    let list_response = pinecone.list_indexes().await;

    assert!(list_response.is_ok());
}