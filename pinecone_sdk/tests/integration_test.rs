use openapi::models::index_model::Metric as OpenApiMetric;
use openapi::models::serverless_spec::Cloud as OpenApiCloud;
use pinecone_sdk::pinecone::control::{Cloud, Metric, WaitPolicy};
use pinecone_sdk::pinecone::data::{Kind, Metadata, SparseValues, Value, Vector};
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::utils::errors::PineconeError;
use std::collections::BTreeMap;
use std::time::Duration;
use std::vec;

// helpers to generate random test/collection names
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

fn generate_namespace_name() -> String {
    format!("test-namespace-{}", generate_random_string())
}

// helper functions to get index names from environment variables
fn get_serverless_index() -> String {
    std::env::var("SERVERLESS_INDEX_NAME").unwrap()
}

fn get_pod_index() -> String {
    std::env::var("POD_INDEX_NAME").unwrap()
}

fn get_collection() -> String {
    std::env::var("COLLECTION_NAME").unwrap()
}

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
        .configure_index(&get_pod_index(), 1, "s1.x1")
        .await
        .expect("Failed to configure index");

    Ok(())
}

#[tokio::test]
async fn test_configure_serverless_index_err() -> Result<(), PineconeError> {
    let pinecone =
        PineconeClient::new(None, None, None, None).expect("Failed to create Pinecone instance");

    let _ = pinecone
        .configure_index(&get_serverless_index(), 1, "p1.x1")
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
async fn test_index() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let host = pinecone
        .describe_index(&get_serverless_index())
        .await
        .unwrap()
        .host;

    let _ = pinecone
        .index(host.as_str())
        .await
        .expect("Failed to target index");

    Ok(())
}

#[tokio::test]
async fn test_index_err() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let _ = pinecone
        .index("invalid-host")
        .await
        .expect_err("Expected to fail targeting index");

    Ok(())
}

#[tokio::test]
async fn test_upsert() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let host = pinecone
        .describe_index(&get_serverless_index())
        .await
        .unwrap()
        .host;

    let mut index = pinecone
        .index(host.as_str())
        .await
        .expect("Failed to target index");

    let vectors = vec![Vector {
        id: "1".to_string(),
        values: vec![1.0, 2.0, 3.0, 5.5],
        sparse_values: None,
        metadata: None,
    }];

    let upsert_response = index.upsert(vectors, None).await.expect("Failed to upsert");

    assert_eq!(upsert_response.upserted_count, 1);

    Ok(())
}

#[tokio::test]
async fn test_describe_index_stats_with_filter() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let host = pinecone
        .describe_index(&get_pod_index())
        .await
        .unwrap()
        .host;

    let mut index = pinecone
        .index(host.as_str())
        .await
        .expect("Failed to target index");

    let mut filter = BTreeMap::new();
    filter.insert(
        "id".to_string(),
        Value {
            kind: Some(Kind::BoolValue(false)),
        },
    );

    let describe_index_stats_response = index
        .describe_index_stats(Some(Metadata { fields: filter }))
        .await
        .expect("Failed to describe index stats");

    assert_eq!(describe_index_stats_response.dimension, 12);

    Ok(())
}

#[tokio::test]
async fn test_describe_index_stats_no_filter() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let host = pinecone
        .describe_index(&get_serverless_index())
        .await
        .unwrap()
        .host;

    let mut index = pinecone
        .index(host.as_str())
        .await
        .expect("Failed to target index");

    let describe_index_stats_response = index
        .describe_index_stats(None)
        .await
        .expect("Failed to describe index stats");

    assert_eq!(describe_index_stats_response.dimension, 4);

    Ok(())
}

#[tokio::test]
async fn test_list_vectors() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let host = pinecone
        .describe_index(&get_serverless_index())
        .await
        .unwrap()
        .host;

    let mut index = pinecone
        .index(host.as_str())
        .await
        .expect("Failed to target index");

    let _list_response = index
        .list("".to_string(), None, None, None)
        .await
        .expect("Failed to list vectors");

    Ok(())
}

#[tokio::test]
async fn test_update_vector() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let host = pinecone
        .describe_index(&get_serverless_index())
        .await
        .unwrap()
        .host;

    let mut index = pinecone
        .index(host.as_str())
        .await
        .expect("Failed to target index");

    let mut metadata = BTreeMap::new();
    metadata.insert(
        "key".to_string(),
        Value {
            kind: Some(Kind::StringValue("value".to_string())),
        },
    );

    let _update_response = index
        .update(
            "valid-vector".to_string(),
            vec![1.0, 2.0, 3.0, 123947.5],
            Some(SparseValues {
                indices: vec![1, 20],
                values: vec![2.0, 3.0],
            }),
            Some(Metadata { fields: metadata }),
            "".to_string(),
        )
        .await
        .expect("Failed to update vector");

    Ok(())
}

#[tokio::test]
async fn test_update_vector_fail_id() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let host = pinecone
        .describe_index(&get_serverless_index())
        .await
        .unwrap()
        .host;

    let mut index = pinecone
        .index(host.as_str())
        .await
        .expect("Failed to target index");

    let _update_response = index
        .update(
            "invalid_id!@*!@&".to_string(),
            vec![1.0, 2.0, 3.0, 5.5],
            None,
            None,
            "namespace".to_string(),
        )
        .await
        .expect_err("Expected to fail updating vector");

    Ok(())
}

#[tokio::test]
async fn test_update_vector_fail_namespace() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let host = pinecone
        .describe_index(&get_serverless_index())
        .await
        .unwrap()
        .host;

    let mut index = pinecone
        .index(host.as_str())
        .await
        .expect("Failed to target index");

    let _update_response = index
        .update(
            "some-id".to_string(),
            vec![1.0, 2.0, 3.0, 5.5],
            None,
            None,
            "invalid-namespace".to_string(),
        )
        .await
        .expect_err("Expected to fail updating vector");

    Ok(())
}

#[tokio::test]
async fn test_delete_vectors_by_ids() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let host = pinecone
        .describe_index(&get_serverless_index())
        .await
        .unwrap()
        .host;

    let mut index = pinecone
        .index(host.as_str())
        .await
        .expect("Failed to target index");

    let vectors = vec![
        Vector {
            id: "1".to_string(),
            values: vec![1.0, 2.0, 3.0, 5.5],
            sparse_values: None,
            metadata: None,
        },
        Vector {
            id: "2".to_string(),
            values: vec![1.0, 2.0, 3.0, 5.5],
            sparse_values: None,
            metadata: None,
        },
    ];

    let namespace = &generate_namespace_name();
    let _ = index
        .upsert(vectors, Some(namespace.to_string()))
        .await
        .expect("Failed to upsert");

    let ids = vec!["1".to_string(), "2".to_string()];

    let _ = index
        .delete_by_id(ids, Some(namespace.to_string()))
        .await
        .expect("Failed to delete vectors by ids");

    Ok(())
}

#[tokio::test]
async fn test_delete_all_vectors() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let host = pinecone
        .describe_index(&get_serverless_index())
        .await
        .unwrap()
        .host;

    let mut index = pinecone
        .index(host.as_str())
        .await
        .expect("Failed to target index");

    let vectors = vec![
        Vector {
            id: "1".to_string(),
            values: vec![1.0, 2.0, 3.0, 5.5],
            sparse_values: None,
            metadata: None,
        },
        Vector {
            id: "2".to_string(),
            values: vec![1.0, 2.0, 3.0, 5.5],
            sparse_values: None,
            metadata: None,
        },
    ];

    let namespace = &generate_namespace_name();
    let _ = index
        .upsert(vectors, Some(namespace.to_string()))
        .await
        .expect("Failed to upsert");

    let _ = index
        .delete_all(Some(namespace.to_string()))
        .await
        .expect("Failed to delete all vectors");

    Ok(())
}

#[tokio::test]
async fn test_delete_by_filter() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let host = pinecone
        .describe_index(&get_pod_index())
        .await
        .unwrap()
        .host;

    let mut index = pinecone
        .index(host.as_str())
        .await
        .expect("Failed to target index");

    let vectors = vec![
        Vector {
            id: "1".to_string(),
            values: vec![1.0; 12],
            sparse_values: None,
            metadata: Some(Metadata {
                fields: vec![(
                    "key".to_string(),
                    Value {
                        kind: Some(Kind::StringValue("value1".to_string())),
                    },
                )]
                .into_iter()
                .collect(),
            }),
        },
        Vector {
            id: "2".to_string(),
            values: vec![2.0; 12],
            sparse_values: None,
            metadata: Some(Metadata {
                fields: vec![(
                    "key".to_string(),
                    Value {
                        kind: Some(Kind::StringValue("value2".to_string())),
                    },
                )]
                .into_iter()
                .collect(),
            }),
        },
    ];

    let namespace = &generate_namespace_name();
    let _ = index
        .upsert(vectors, Some(namespace.to_string()))
        .await
        .expect("Failed to upsert");

    let filter = Metadata {
        fields: vec![(
            "key".to_string(),
            Value {
                kind: Some(Kind::StringValue("value1".to_string())),
            },
        )]
        .into_iter()
        .collect(),
    };

    let _ = index
        .delete_by_filter(filter, Some(namespace.to_string()))
        .await
        .expect("Failed to delete all vectors");

    Ok(())
}

#[tokio::test]
async fn test_fetch_vectors() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let host = pinecone
        .describe_index(&get_serverless_index())
        .await
        .unwrap()
        .host;

    let mut index = pinecone
        .index(host.as_str())
        .await
        .expect("Failed to target index");

    let vectors = vec![
        Vector {
            id: "1".to_string(),
            values: vec![5.0, 6.0, 7.0, 8.0],
            sparse_values: None,
            metadata: None,
        },
        Vector {
            id: "2".to_string(),
            values: vec![9.0, 10.0, 11.0, 12.0],
            sparse_values: None,
            metadata: None,
        },
    ];

    let namespace = &generate_namespace_name();
    eprintln!("namespace: {}", namespace);
    //let namespace = "test_namespace";

    let _ = index
        .upsert(vectors, Some(namespace.to_string()))
        .await
        .expect("Failed to upsert");

    std::thread::sleep(std::time::Duration::from_secs(5));

    let fetch_response = index
        .fetch(
            vec!["1".to_string(), "2".to_string()],
            Some(namespace.to_string()),
        )
        .await
        .expect("Failed to fetch vectors");

    assert_eq!(fetch_response.namespace, namespace.to_string());
    let vectors = fetch_response.vectors;
    assert_eq!(
        *vectors.get("1").unwrap(),
        Vector {
            id: "1".to_string(),
            values: vec![5.0, 6.0, 7.0, 8.0],
            sparse_values: None,
            metadata: None,
        }
    );
    assert_eq!(
        *vectors.get("2").unwrap(),
        Vector {
            id: "2".to_string(),
            values: vec![9.0, 10.0, 11.0, 12.0],
            sparse_values: None,
            metadata: None,
        }
    );

    let _ = index
        .delete_all(Some(namespace.to_string()))
        .await
        .expect("Failed to delete all vectors");

    Ok(())
}

#[tokio::test]
async fn test_fetch_no_match() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let host = pinecone
        .describe_index(&get_serverless_index())
        .await
        .unwrap()
        .host;

    let mut index = pinecone
        .index(host.as_str())
        .await
        .expect("Failed to target index");

    let fetch_response = index
        .fetch(vec!["invalid-id1".to_string(), "invalid-id2".to_string()], None)
        .await
        .expect("Failed to fetch vectors");

    assert_eq!(fetch_response.namespace, "");
    assert_eq!(fetch_response.vectors.len(), 0);

    Ok(())
}

#[tokio::test]
async fn test_fetch_empty_id_list() -> Result<(), PineconeError> {
    let pinecone = PineconeClient::new(None, None, None, None).unwrap();

    let host = pinecone
        .describe_index(&get_serverless_index())
        .await
        .unwrap()
        .host;

    let mut index = pinecone
        .index(host.as_str())
        .await
        .expect("Failed to target index");

    let _ = index
        .fetch(vec![], None)
        .await
        .expect_err("Expected error to be thrown");

    Ok(())
}
