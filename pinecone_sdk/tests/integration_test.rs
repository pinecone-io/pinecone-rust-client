use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::control::{Cloud, Metric};
use pinecone_sdk::utils::errors::PineconeError;

#[tokio::test]
async fn test_list_index() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    let _ = pinecone
        .list_indexes()
        .await
        .expect("Failed to list indexes");

    Ok(())
}

#[tokio::test]
async fn test_create_delete_index() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    
    let name = "test-index";
    let dimension = 2;
    let metric = Metric::Euclidean;
    let cloud = Cloud::Aws;
    let region = "us-west-2";

    let response = pinecone
        .create_serverless_index(name, dimension, metric, cloud, region)
        .await;

    assert!(response.is_ok());
    let response = response.unwrap();
    assert_eq!(response.name, name);
    assert_eq!(response.dimension, 2);
    assert_eq!(response.metric, openapi::models::index_model::Metric::Euclidean);

    let spec = response.spec.serverless.unwrap();
    assert_eq!(spec.cloud, openapi::models::serverless_spec::Cloud::Aws);
    assert_eq!(spec.region, "us-west-2");

    Ok(())
}