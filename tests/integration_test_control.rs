use common::{generate_index_name, get_serverless_index};
use pinecone_sdk::models::{Cloud, DeletionProtection, Metric, VectorType, WaitPolicy};
use pinecone_sdk::pinecone::default_client;
use pinecone_sdk::utils::errors::PineconeError;
use serial_test::serial;

mod common;

#[tokio::test]
async fn test_describe_index() -> Result<(), PineconeError> {
    // get environment variables

    let pinecone = default_client().expect("Failed to create Pinecone instance");

    pinecone
        .describe_index(&get_serverless_index())
        .await
        .expect("Failed to describe index");

    Ok(())
}

#[tokio::test]
async fn test_describe_index_fail() -> Result<(), PineconeError> {
    let pinecone = default_client().expect("Failed to create Pinecone instance");

    pinecone
        .describe_index("invalid-index")
        .await
        .expect_err("Expected to fail describing index");

    Ok(())
}

#[tokio::test]
async fn test_create_list_indexes() -> Result<(), PineconeError> {
    let pinecone = default_client().expect("Failed to create Pinecone instance");

    let index1_name = &generate_index_name();
    let index2_name = &generate_index_name();

    pinecone
        .create_serverless_index(
            index1_name,
            2,
            Default::default(),
            Cloud::Aws,
            "us-west-2",
            DeletionProtection::Disabled,
            WaitPolicy::NoWait,
            VectorType::Dense,
            None,
        )
        .await
        .expect("Failed to create index");

    pinecone
        .create_serverless_index(
            index2_name,
            2,
            Metric::Dotproduct,
            Cloud::Aws,
            "us-west-2",
            DeletionProtection::Disabled,
            WaitPolicy::NoWait,
            VectorType::Dense,
            None,
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
        .find(|index| index.name == *index1_name)
        .unwrap();

    assert_eq!(index1.name, index1_name.to_string());
    assert_eq!(index1.dimension, Some(2));
    assert_eq!(index1.metric, Metric::Cosine);
    let spec1 = index1.spec.serverless.as_ref().unwrap();
    let spec1_cloud: Cloud = spec1.cloud.into();
    assert_eq!(spec1_cloud, Cloud::Aws);
    assert_eq!(spec1.region, "us-west-2");

    let index2 = indexes
        .iter()
        .find(|index| index.name == *index2_name)
        .unwrap();

    assert_eq!(index2.name, index2_name.to_string());
    assert_eq!(index2.dimension, Some(2));
    assert_eq!(index2.metric, Metric::Dotproduct);
    let spec2 = index2.spec.serverless.as_ref().unwrap();
    let spec2_cloud: Cloud = spec2.cloud.into();
    assert_eq!(spec2_cloud, Cloud::Aws);
    assert_eq!(spec2.region, "us-west-2");

    pinecone
        .delete_index(index1_name)
        .await
        .expect("Failed to delete index");

    pinecone
        .delete_index(index2_name)
        .await
        .expect("Failed to delete index");

    Ok(())
}

#[tokio::test]
async fn test_create_delete_index() -> Result<(), PineconeError> {
    let pinecone = default_client().expect("Failed to create Pinecone instance");

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
            VectorType::Dense,
            None,
        )
        .await
        .expect("Failed to create index");

    assert_eq!(response.name, name.to_string());
    assert_eq!(response.dimension, Some(2));
    assert_eq!(response.metric, Metric::Euclidean);

    let spec = response.spec.serverless.unwrap();
    let spec_cloud: Cloud = spec.cloud.into();
    assert_eq!(spec_cloud, Cloud::Aws);
    assert_eq!(spec.region, "us-west-2");

    pinecone
        .delete_index(name)
        .await
        .expect("Failed to delete index");

    Ok(())
}

#[tokio::test]
async fn test_delete_index_err() -> Result<(), PineconeError> {
    let pinecone = default_client().expect("Failed to create Pinecone instance");

    pinecone
        .delete_index("invalid-index")
        .await
        .expect_err("Expected to fail deleting invalid index");

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_configure_index() -> Result<(), PineconeError> {
    let pinecone = default_client().expect("Failed to create Pinecone instance");

    pinecone
        .configure_index(
            &get_serverless_index(),
            Some(DeletionProtection::Enabled),
            None,
            None,
            None,
            None,
        )
        .await
        .expect("Failed to configure index");

    Ok(())
}

#[tokio::test]
async fn test_configure_deletion_protection() -> Result<(), PineconeError> {
    let pinecone = default_client().expect("Failed to create Pinecone instance");

    let index_name = &generate_index_name();
    pinecone
        .create_serverless_index(
            index_name,
            2,
            Default::default(),
            Cloud::Aws,
            "us-east-1",
            DeletionProtection::Enabled,
            WaitPolicy::NoWait,
            VectorType::Dense,
            None,
        )
        .await
        .expect("Failed to create index");

    pinecone
        .delete_index(index_name)
        .await
        .expect_err("Expected to fail to delete index");

    pinecone
        .configure_index(
            index_name,
            Some(DeletionProtection::Disabled),
            None,
            None,
            None,
            None,
        )
        .await
        .expect("Failed to configure index");

    pinecone
        .delete_index(index_name)
        .await
        .expect("Failed to delete index");

    Ok(())
}

#[tokio::test]
async fn test_configure_serverless_index_err() -> Result<(), PineconeError> {
    let pinecone = default_client().expect("Failed to create Pinecone instance");

    pinecone
        .configure_index(
            &get_serverless_index(),
            Some(DeletionProtection::Enabled),
            Some(1),
            Some("p1.x1"),
            None,
            None,
        )
        .await
        .expect_err("Expected to fail configuring serverless index");

    Ok(())
}

#[tokio::test]
async fn test_configure_invalid_index_err() -> Result<(), PineconeError> {
    let pinecone = default_client().expect("Failed to create Pinecone instance");

    pinecone
        .configure_index(
            "invalid-index",
            Some(DeletionProtection::Enabled),
            Some(2),
            Some("p1.x1"),
            None,
            None,
        )
        .await
        .expect_err("Expected to fail configuring invalid index");

    Ok(())
}
