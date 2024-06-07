use pinecone_sdk::pinecone::Pinecone;

#[tokio::test]
async fn test_list_serverless_index() {
    let pinecone = Pinecone::new("b41b6453-9756-45aa-8d8d-a51c295d3c78".to_string(), None);
    let list_response = pinecone.list_indexes().await;

    match list_response {
        Ok(index_list) => {
            match index_list.indexes {
                Some(indexes) => {
                    assert_eq!(indexes.len(), 0);
                }
                None => {
                    assert!(false, "Expected indexes to be Some");
                }
            }
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
}
