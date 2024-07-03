use std::vec;

use openapi::models::index_model::Metric as OpenApiMetric;
use openapi::models::serverless_spec::Cloud as OpenApiCloud;
use pinecone_sdk::pinecone::control::{Cloud, Metric, WaitPolicy};
use pinecone_sdk::pinecone::data::Vector;
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::utils::errors::PineconeError;

fn generate_random_string() -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let s: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    s.to_lowercase()
}

fn generate_index_name() -> String {
    format!("test-index-{}", generate_random_string())
}

fn generate_collection_name() -> String {
    format!("test-collection-{}", generate_random_string())
}

#[tokio::test]
async fn test_describe_index() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");
    let _ = pinecone
        .describe_index("valid-index")
        .await
        .expect("Failed to describe index");

    Ok(())
}

#[tokio::test]
async fn test_describe_index_fail() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");
    let _ = pinecone
        .describe_index("invalid-index")
        .await
        .expect_err("Expected to fail describing index");

    Ok(())
}

#[tokio::test]
async fn test_create_list_indexes() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let index1_name = &generate_index_name();
    let index2_name = &generate_index_name();

    let _ = pinecone
        .create_serverless_index(
            index1_name,
            2,
            Metric::Cosine,
            Cloud::Aws,
            "us-west-2",
            WaitPolicy::NoWait,
        )
        .await
        .expect("Failed to create index");

    let _ = pinecone
        .create_serverless_index(
            index2_name,
            2,
            Metric::Dotproduct,
            Cloud::Aws,
            "us-west-2",
            WaitPolicy::NoWait,
        )
        .await
        .expect("Failed to create index");

    let index_list = pinecone
        .list_indexes()
        .await
        .expect("Failed to list indexes");
    let indexes = index_list.indexes.unwrap();

    let index1 = indexes
        .iter()
        .find(|index| index.name == index1_name.to_string())
        .unwrap();

    assert_eq!(index1.name, index1_name.to_string());
    assert_eq!(index1.dimension, 2);
    assert_eq!(index1.metric, OpenApiMetric::Cosine);
    let spec1 = index1.spec.serverless.as_ref().unwrap();
    assert_eq!(spec1.cloud, OpenApiCloud::Aws);
    assert_eq!(spec1.region, "us-west-2");

    let index2 = indexes
        .iter()
        .find(|index| index.name == index2_name.to_string())
        .unwrap();

    assert_eq!(index2.name, index2_name.to_string());
    assert_eq!(index2.dimension, 2);
    assert_eq!(index2.metric, OpenApiMetric::Dotproduct);
    let spec2 = index2.spec.serverless.as_ref().unwrap();
    assert_eq!(spec2.cloud, OpenApiCloud::Aws);
    assert_eq!(spec2.region, "us-west-2");

    let _ = pinecone
        .delete_index(index1_name)
        .await
        .expect("Failed to delete index");

    let _ = pinecone
        .delete_index(index2_name)
        .await
        .expect("Failed to delete index");

    Ok(())
}

#[tokio::test]
async fn test_create_delete_index() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let name = &generate_index_name();

    let response = pinecone
        .create_serverless_index(
            name,
            2,
            Metric::Euclidean,
            Cloud::Aws,
            "us-west-2",
            WaitPolicy::NoWait,
        )
        .await
        .expect("Failed to create index");

    assert_eq!(response.name, name.to_string());
    assert_eq!(response.dimension, 2);
    assert_eq!(response.metric, OpenApiMetric::Euclidean);

    let spec = response.spec.serverless.unwrap();
    assert_eq!(spec.cloud, OpenApiCloud::Aws);
    assert_eq!(spec.region, "us-west-2");

    let _ = pinecone
        .delete_index(name)
        .await
        .expect("Failed to delete index");

    Ok(())
}

#[tokio::test]
async fn test_create_pod_index() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let name = &generate_index_name();

    let response = pinecone
        .create_pod_index(
            name,
            2,
            Metric::Euclidean,
            "us-west1-gcp",
            "p1.x1",
            1,
            Some(1),
            Some(1),
            None,
            None,
            WaitPolicy::NoWait,
        )
        .await
        .expect("Failed to create index");

    assert_eq!(response.name, name.to_string());
    assert_eq!(response.dimension, 2);
    assert_eq!(response.metric, OpenApiMetric::Euclidean);

    let spec = response.spec.pod.unwrap();
    assert_eq!(spec.environment, "us-west1-gcp");
    assert_eq!(spec.replicas, Some(1));
    assert_eq!(spec.shards, Some(1));
    assert_eq!(spec.pod_type, "p1.x1");
    assert_eq!(spec.pods, 1);
    assert_eq!(spec.source_collection, None);

    let _ = pinecone
        .delete_index(name)
        .await
        .expect("Failed to delete index");

    Ok(())
}

#[tokio::test]
async fn test_create_pod_index_collection() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let name = &generate_index_name();

    let response = pinecone
        .create_pod_index(
            name,
            12,
            Metric::Euclidean,
            "us-east-1-aws",
            "p1.x1",
            1,
            Some(1),
            Some(1),
            None,
            Some("valid-collection"),
            WaitPolicy::NoWait,
        )
        .await
        .expect("Failed to create index");

    assert_eq!(response.name, name.to_string());
    assert_eq!(response.dimension, 12);
    assert_eq!(response.metric, OpenApiMetric::Euclidean);

    let spec = response.spec.pod.unwrap();
    assert_eq!(spec.environment, "us-east-1-aws");
    assert_eq!(spec.replicas, Some(1));
    assert_eq!(spec.shards, Some(1));
    assert_eq!(spec.pod_type, "p1.x1");
    assert_eq!(spec.pods, 1);
    assert_eq!(spec.source_collection, Some("valid-collection".to_string()));

    let _ = pinecone
        .delete_index(name)
        .await
        .expect("Failed to delete index");

    Ok(())
}

#[tokio::test]
async fn test_delete_index_err() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let _ = pinecone
        .delete_index("invalid-index")
        .await
        .expect_err("Expected to fail deleting invalid index");

    Ok(())
}

#[tokio::test]
async fn test_configure_index() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let _ = pinecone
        .configure_index("valid-index-pod", 1, "s1.x1")
        .await
        .expect("Failed to configure index");

    Ok(())
}

#[tokio::test]
async fn test_configure_serverless_index_err() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let _ = pinecone
        .configure_index("valid-index", 1, "p1.x1")
        .await
        .expect_err("Expected to fail configuring serverless index");

    Ok(())
}

#[tokio::test]
async fn test_configure_invalid_index_err() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let _ = pinecone
        .configure_index("invalid-index", 2, "p1.x1")
        .await
        .expect_err("Expected to fail configuring invalid index");

    Ok(())
}

#[tokio::test]
async fn test_list_collections() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");
    let _ = pinecone
        .list_collections()
        .await
        .expect("Failed to list collections");

    Ok(())
}

#[tokio::test]
async fn test_create_delete_collection() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let collection_name = generate_collection_name();

    let response = pinecone
        .create_collection(&collection_name, "valid-index-pod")
        .await
        .expect("Failed to create collection");

    assert_eq!(response.name, collection_name.to_string());

    let _ = pinecone
        .delete_collection(&collection_name)
        .await
        .expect("Failed to delete collection");

    Ok(())
}

#[tokio::test]
async fn test_create_collection_serverless_err() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let collection_name = generate_collection_name();

    let _ = pinecone
        .create_collection(&collection_name, "valid-index")
        .await
        .expect_err("Expected to fail creating collection from serverless");

    Ok(())
}

#[tokio::test]
async fn test_create_collection_invalid_err() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let collection_name = generate_collection_name();

    let _ = pinecone
        .create_collection(&collection_name, "invalid-index")
        .await
        .expect_err("Expected to fail creating collection from invalid index");

    Ok(())
}

#[tokio::test]
async fn test_delete_collection_err() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let _ = pinecone
        .delete_collection("invalid-collection")
        .await
        .expect_err("Expected to fail deleting collection");

    Ok(())
}

#[tokio::test]
async fn test_upsert() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let mut index = pinecone
        .index("test-data-plane")
        .await
        .expect("Failed to target index");

    let vectors = vec![Vector {
        id: "1".to_string(),
        values: vec![1.0, 2.0, 3.0, 5.5],
        sparse_values: None,
        metadata: None,
    }]; // Convert inner vector to Vector

    let upsert_response = index.upsert(vectors, None).await.expect("Failed to upsert");

    assert_eq!(upsert_response.upserted_count, 1);

    Ok(())
}
