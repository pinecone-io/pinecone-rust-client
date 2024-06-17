use pinecone_sdk::control::{Cloud, Metric};
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::utils::errors::PineconeError;

fn generate_index_name() -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    format!("test-index-{}", rand_string.to_lowercase())
}

#[tokio::test]
async fn test_describe_index() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    let _ = pinecone
        .describe_index("valid-index")
        .await
        .expect("Failed to describe index");

    Ok(())
}

#[tokio::test]
async fn test_describe_index_fail() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    let _ = pinecone
        .describe_index("invalid-index")
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

#[tokio::test]
async fn test_create_delete_index() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let name = &generate_index_name();
    println!("Generated index name: {}", name);

    let dimension = 2;
    let metric = Metric::Euclidean;
    let cloud = Cloud::Aws;
    let region = "us-west-2";

    let response = pinecone
        .create_serverless_index(name, dimension, metric, cloud, region)
        .await
        .expect("Failed to create index");

    assert_eq!(response.name, name.to_string());
    assert_eq!(response.dimension, 2);
    assert_eq!(
        response.metric,
        openapi::models::index_model::Metric::Euclidean
    );

    let spec = response.spec.serverless.unwrap();
    assert_eq!(spec.cloud, openapi::models::serverless_spec::Cloud::Aws);
    assert_eq!(spec.region, "us-west-2");

    let _ = pinecone
        .delete_index(name)
        .await
        .expect("Failed to delete index");

    Ok(())
}

#[tokio::test]
async fn test_delete_index_err() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    let name = "invalid-index";
    let response = pinecone.delete_index(name).await;
    assert!(response.is_err());
    Ok(())
}
