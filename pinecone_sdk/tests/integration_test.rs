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
async fn test_create_list_indexes() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let index1_name = &generate_index_name();
    let index2_name = &generate_index_name();

    let _ = pinecone
        .create_serverless_index(index1_name, 2, Metric::Cosine, Cloud::Aws, "us-west-2")
        .await
        .expect("Failed to create index");
    let _ = pinecone
        .create_serverless_index(index2_name, 2, Metric::Dotproduct, Cloud::Aws, "us-west-2")
        .await
        .expect("Failed to create index");

    let index_list = pinecone
        .list_indexes()
        .await
        .expect("Failed to list indexes");
    let indexes = index_list.indexes.unwrap();

    println!("Indexes: {:?}", indexes);

    let index1 = indexes.iter().find(|index| index.name == index1_name.to_string()).unwrap();
    assert_eq!(index1.name, index1_name.to_string());
    assert_eq!(index1.dimension, 2);
    assert_eq!(index1.metric, openapi::models::index_model::Metric::Cosine);
    let spec1 = index1.spec.serverless.as_ref().unwrap();
    assert_eq!(spec1.cloud, openapi::models::serverless_spec::Cloud::Aws);
    assert_eq!(spec1.region, "us-west-2");

    let index2 = indexes.iter().find(|index| index.name == index2_name.to_string()).unwrap();
    assert_eq!(index2.name, index2_name.to_string());
    assert_eq!(index2.dimension, 2);
    assert_eq!(
        index2.metric,
        openapi::models::index_model::Metric::Dotproduct
    );
    let spec2 = index2.spec.serverless.as_ref().unwrap();
    assert_eq!(spec2.cloud, openapi::models::serverless_spec::Cloud::Aws);
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
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let name = &generate_index_name();

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
async fn test_create_pod_index() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let name = &generate_index_name();
    let dimension = 2;
    let metric = Metric::Euclidean;
    let environment = "us-west1-gcp";
    let replicas = Some(1);
    let shards = Some(1);
    let pod_type = "p1.x1";
    let pods = 1;
    let indexed = None;
    let source_collection = None;

    let response = pinecone
        .create_pod_index(
            name,
            dimension,
            metric,
            environment,
            pod_type,
            pods,
            replicas,
            shards,
            indexed,
            source_collection,
        )
        .await
        .expect("Failed to create index");

    assert_eq!(response.name, name.to_string());
    assert_eq!(response.dimension, 2);
    assert_eq!(
        response.metric,
        openapi::models::index_model::Metric::Euclidean
    );

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
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    let name = &generate_index_name();
    let dimension = 12;
    let metric = Metric::Euclidean;
    let environment = "us-east-1-aws";
    let replicas = Some(1);
    let shards = Some(1);
    let pod_type = "p1.x1";
    let pods = 1;
    let indexed = None;
    let source_collection = Some("valid-collection");

    let response = pinecone
        .create_pod_index(
            name,
            dimension,
            metric,
            environment,
            pod_type,
            pods,
            replicas,
            shards,
            indexed,
            source_collection,
        )
        .await
        .expect("Failed to create index");

    assert_eq!(response.name, name.to_string());
    assert_eq!(response.dimension, 12);
    assert_eq!(
        response.metric,
        openapi::models::index_model::Metric::Euclidean
    );

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
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let _ = pinecone
        .delete_index("invalid-index")
        .await
        .expect_err("Expected to fail deleting invalid index");

    Ok(())
}

#[tokio::test]
async fn test_configure_index() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let _ = pinecone
        .configure_index("valid-index-pod", 1, "s1.x1")
        .await
        .expect("Failed to configure index");

    Ok(())
}

#[tokio::test]
async fn test_configure_serverless_index_err() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let _ = pinecone
        .configure_index("valid-index", 1, "p1.x1")
        .await
        .expect_err("Expected to fail configuring serverless index");

    Ok(())
}

#[tokio::test]
async fn test_configure_invalid_index_err() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let _ = pinecone
        .configure_index("invalid-index", 2, "p1.x1")
        .await
        .expect_err("Expected to fail configuring invalid index");

    Ok(())
}

#[tokio::test]
async fn test_list_collections() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    let _ = pinecone
        .list_collections()
        .await
        .expect("Failed to list collections");

    Ok(())
}
