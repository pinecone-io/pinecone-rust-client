use common::{
    generate_collection_name, generate_index_name, get_collection, get_pod_index,
    get_serverless_index,
};
use pinecone_sdk::openapi::models::index_model::Metric as OpenApiMetric;
use pinecone_sdk::openapi::models::serverless_spec::Cloud as OpenApiCloud;
use pinecone_sdk::pinecone::control::{Cloud, DeletionProtection, Metric, WaitPolicy};
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::utils::errors::PineconeError;
use std::collections::HashMap;
use std::time::Duration;

mod common;

#[tokio::test]
async fn test_describe_index() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let _ = pinecone
        .describe_index(&get_serverless_index())
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
            DeletionProtection::Disabled,
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
            DeletionProtection::Disabled,
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
            DeletionProtection::Disabled,
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
            1,
            1,
            DeletionProtection::Disabled,
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
    assert_eq!(spec.replicas, 1);
    assert_eq!(spec.shards, 1);
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
            1,
            1,
            DeletionProtection::Disabled,
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
    assert_eq!(spec.replicas, 1);
    assert_eq!(spec.shards, 1);
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
        .configure_index(
            &get_pod_index(),
            Some(DeletionProtection::Enabled),
            Some(1),
            Some("s1.x1"),
        )
        .await
        .expect("Failed to configure index");

    Ok(())
}

#[tokio::test]
async fn test_configure_deletion_protection() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let index_name = &generate_index_name();
    let _ = pinecone
        .create_serverless_index(
            index_name,
            2,
            Metric::Cosine,
            Cloud::Aws,
            "us-east-1",
            DeletionProtection::Enabled,
            WaitPolicy::NoWait,
        )
        .await
        .expect("Failed to create index");

    let _ = pinecone
        .delete_index(index_name)
        .await
        .expect_err("Expected to fail to delete index");

    let _ = pinecone
        .configure_index(index_name, Some(DeletionProtection::Disabled), None, None)
        .await
        .expect("Failed to configure index");

    let _ = pinecone
        .delete_index(&index_name)
        .await
        .expect("Failed to delete index");

    Ok(())
}

#[tokio::test]
async fn test_configure_optional_deletion_prot() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let index_name = &generate_index_name();
    let _ = pinecone
        .create_pod_index(
            index_name,
            2,
            Metric::Cosine,
            "us-east-1-aws",
            "p1.x1",
            1,
            1,
            1,
            DeletionProtection::Enabled,
            None,
            None,
            WaitPolicy::NoWait,
        )
        .await
        .expect("Failed to create index");

    let _ = pinecone
        .configure_index(index_name, None, Some(2), None)
        .await
        .expect("Failed to configure index");

    let response = pinecone
        .delete_index(index_name)
        .await
        .expect_err("Expected to fail to delete index");

    assert!(matches!(
        response,
        PineconeError::ActionForbiddenError { source: _ }
    ));

    let _ = pinecone
        .configure_index(index_name, Some(DeletionProtection::default()), None, None)
        .await
        .expect("Failed to configure index");

    let _ = pinecone
        .delete_index(index_name)
        .await
        .expect("Failed to delete collection");

    Ok(())
}

#[tokio::test]
async fn test_configure_serverless_index_err() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let _ = pinecone
        .configure_index(
            &get_serverless_index(),
            Some(DeletionProtection::Enabled),
            Some(1),
            Some("p1.x1"),
        )
        .await
        .expect_err("Expected to fail configuring serverless index");

    Ok(())
}

#[tokio::test]
async fn test_configure_invalid_index_err() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let _ = pinecone
        .configure_index(
            "invalid-index",
            Some(DeletionProtection::Enabled),
            Some(2),
            Some("p1.x1"),
        )
        .await
        .expect_err("Expected to fail configuring invalid index");

    Ok(())
}

#[tokio::test]
async fn test_create_delete_collection() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let collection_name = generate_collection_name();

    let index_name = &get_pod_index();
    loop {
        if match pinecone.describe_index(index_name).await {
            Ok(index) => index.status.ready,
            Err(_) => false,
        } {
            break;
        }
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }

    let response = pinecone
        .create_collection(&collection_name, index_name)
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
        .create_collection(&collection_name, &get_serverless_index())
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
async fn test_describe_collection() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let collection_name = &get_collection();

    let _ = pinecone
        .describe_collection(&collection_name)
        .await
        .expect("Failed to describe collection");

    Ok(())
}

#[tokio::test]
async fn test_describe_collection_fail() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let _ = pinecone
        .describe_collection("invalid-collection")
        .await
        .expect_err("Expected to fail describing collection");

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
async fn test_list_collections_invalid_api_version() -> Result<(), PineconeError> {
    let headers: HashMap<String, String> = [(
        pinecone_sdk::pinecone::PINECONE_API_VERSION_KEY.to_string(),
        "invalid".to_string(),
    )]
    .iter()
    .cloned()
    .collect();

    let pinecone = PineconeClient::new(None, None, Some(headers), None)
        .expect("Failed to create Pinecone instance");

    let _ = pinecone
        .list_collections()
        .await
        .expect_err("Expected to fail listing collections due to invalid api version");

    Ok(())
}

#[tokio::test]
async fn test_delete_collection_invalid_collection() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let _ = pinecone
        .delete_collection("invalid-collection")
        .await
        .expect_err("Expected to fail deleting collection");

    Ok(())
}