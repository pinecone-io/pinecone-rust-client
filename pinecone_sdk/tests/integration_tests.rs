use pinecone_sdk::pinecone::Pinecone;
use std::env;

#[tokio::test]
async fn test_list_serverless_index() {
    let api_key = env::var("PINECONE_API_KEY").unwrap();
    let pinecone = Pinecone::new(api_key.to_string(), None);
    let list_response = pinecone.list_indexes().await;

    match list_response {
        Ok(index_list) => {
            if let Some(indexes) = index_list.indexes {
                assert_eq!(indexes.len(), 0);
            } else {
                panic!("No indexes found");
            }
        },
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
}
