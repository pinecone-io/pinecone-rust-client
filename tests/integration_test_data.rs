use common::{generate_namespace_name, generate_vector, get_pod_index, get_serverless_index};
use pinecone_sdk::models::{Kind, Metadata, Namespace, SparseValues, Value, Vector};
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::utils::errors::PineconeError;
use std::collections::BTreeMap;
use std::vec;

mod common;

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

    let vectors = &[Vector {
        id: "1".to_string(),
        values: vec![1.0, 2.0, 3.0, 5.5],
        sparse_values: None,
        metadata: None,
    }];

    let upsert_response = index
        .upsert(vectors, &Default::default())
        .await
        .expect("Failed to upsert");

    assert_eq!(upsert_response.upserted_count, 1);

    Ok(())
}

#[tokio::test]
async fn test_upsert_sliced_vectors() -> Result<(), PineconeError> {
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

    let mut vectors = vec![];

    for i in 0..100 {
        vectors.push(Vector {
            id: i.to_string(),
            values: generate_vector(4),
            sparse_values: None,
            metadata: None,
        });
    }

    let mut upserted_count = 0;

    for i in 0..10 {
        let slice = &vectors[i * 10..(i + 1) * 10];
        let upsert_response = index
            .upsert(slice, &Default::default())
            .await
            .expect("Failed to upsert");

        upserted_count += upsert_response.upserted_count;
    }

    assert_eq!(upserted_count, 100);

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
        .list(&Default::default(), None, None, None)
        .await
        .expect("Failed to list vectors");

    Ok(())
}

#[tokio::test]
async fn test_query_by_id() -> Result<(), PineconeError> {
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

    let _query_response = index
        .query_by_id("1", 10, &Namespace::default(), None, None, None)
        .await
        .expect("Failed to query");

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
            "valid-vector",
            vec![1.0, 2.0, 3.0, 123947.5],
            Some(SparseValues {
                indices: vec![1, 20],
                values: vec![2.0, 3.0],
            }),
            Some(Metadata { fields: metadata }),
            &Default::default(),
        )
        .await
        .expect("Failed to update vector");

    Ok(())
}

#[tokio::test]
async fn test_query_by_value() -> Result<(), PineconeError> {
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

    let vector = vec![1.0, 2.0, 3.0, 5.5];

    let _query_response = index
        .query_by_value(vector, None, 10, &Namespace::default(), None, None, None)
        .await
        .expect("Failed to query");

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
            "invalid_id!@*!@&",
            vec![1.0, 2.0, 3.0, 5.5],
            None,
            None,
            &Namespace::from("test-namespace"),
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
            "some-id",
            vec![1.0, 2.0, 3.0, 5.5],
            None,
            None,
            &Namespace::from("invalid-namespace"),
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

    let vectors = &[
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
        .upsert(vectors, namespace)
        .await
        .expect("Failed to upsert");

    let ids = &["1", "2"];

    let _ = index
        .delete_by_id(ids, namespace)
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

    let vectors = &[
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
        .upsert(vectors, namespace)
        .await
        .expect("Failed to upsert");

    let _ = index
        .delete_all(namespace)
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

    let vectors = &[
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
        .upsert(vectors, namespace)
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
        .delete_by_filter(filter, namespace)
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

    let vectors = &[
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

    let _ = index
        .upsert(vectors, namespace)
        .await
        .expect("Failed to upsert");

    std::thread::sleep(std::time::Duration::from_secs(5));

    let fetch_response = index
        .fetch(&["1", "2"], namespace)
        .await
        .expect("Failed to fetch vectors");

    assert_eq!(fetch_response.namespace, namespace.name);
    assert_eq!(fetch_response.vectors.len(), 2);
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
        .delete_all(namespace)
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
        .fetch(&["invalid-id1", "invalid-id2"], &Default::default())
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
        .fetch(&[], &Default::default())
        .await
        .expect_err("Expected error to be thrown");

    Ok(())
}
