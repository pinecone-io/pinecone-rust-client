# Pinecone Rust SDK

[license information]

## Documentation

[reference the documentation here]

### Example code

[reference the sample app]

## Prerequisites

Rust version?

Before you can use the Pinecone SDK, you must sign up for an account and find your API key in the Pinecone console dashboard at [https://app.pinecone.io](https://app.pinecone.io).

## Installation

How to install - instruction for getting the package from crates.io

## Usage

Explanation about how environment variables are used

Proxy config?

# Indexes

## Create Index

### Create serverless index
The following example creates a serverless index in the `us-east-1` region of AWS. For more information on serverless and regional availability, see [Understanding indexes](https://docs.pinecone.io/guides/indexes/understanding-indexes#serverless-indexes)
```rust
use pinecone_sdk::pinecone::{PineconeClient, control::{Metric, Cloud, WaitPolicy}};

let pinecone = PineconeClient::new('<<PINECONE_API_KEY>>', None, None, None).unwrap();
 
pinecone.create_serverless_index(
    "index-name", // Name of the index
    10, // Dimension of the vectors
    Metric::Cosine, // Distance metric
    Cloud::Aws, // Cloud provider
    "us-east-1", // Region
    WaitPolicy::NoWait // Timeout
).await;
```

### Create pod index
The following example creates a pod index in the `us-east-1` region of AWS. This example does not create replicas, or shards, nor use metadata or a source collection.
```rust
use pinecone_sdk::pinecone::{PineconeClient, control::{Metric, Cloud, WaitPolicy}};

let pinecone = PineconeClient::new('<<PINECONE_API_KEY>>', None, None, None).unwrap();

pinecone.create_pod_index(
    "index-name",
    10,
    Metric::Cosine,
    "us-east-1",
    1,
    None,
    None,
    None,
    None,
    WaitPolicy::NoWait
).await;
```

## List indexes
```rust
use pinecone_sdk::pinecone::{ClientClient, control::IndexList};

let pinecone = PineconeClient::new('<<PINECONE_API_KEY>>', None, None, None).unwrap();

pinecone.list_indexes().await;
```

## Describe index
```rust
use pinecone_sdk::pinecone::{PineconeClient, control::IndexModel};

let pinecone = PineconeClient::new('<<PINECONE_API_KEY>>', None, None, None).unwrap();

pinecone.describe_index("index-name").await;
```

## Configure index
```rust
use pinecone_sdk::pinecone::PineconeClient;

let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    
pinecone.configure_index("index-name", 6, "s1").await;
```

## Delete index
```rust
use pinecone_sdk::pinecone::PineconeClient;

let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    
pinecone.delete_index("index-name").await;
```

## Describe index statistics
Without filter
```rust
use std::collections::BTreeMap;
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::pinecone::data::{Value, Kind, Metadata, Namespace};

let pinecone = PineconeClient::new(None, None, None, None).unwrap();

let mut index = pinecone.index("index-host").await.unwrap();

index.describe_index_stats(None).await.unwrap();
```

With filter
```rust
use std::collections::BTreeMap;
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::pinecone::data::{Value, Kind, Metadata, Namespace};

let pinecone = PineconeClient::new(None, None, None, None).unwrap();

let mut index = pinecone.index("index-host").await.unwrap();

let mut fields = BTreeMap::new();
fields.insert("field".to_string(), Value { kind: Some(Kind::StringValue("value".to_string()))});

index.describe_index_stats(Some(Metadata { fields })).await.unwrap();
```

## Upsert vectors
```rust
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::pinecone::data::Vector;

let pinecone = PineconeClient::new(None, None, None, None).unwrap();

let mut index = pinecone.index("index-host").await.unwrap();

let vectors = [Vector {
    id: "id1".to_string(),
    values: vec![1.0, 2.0, 3.0, 4.0],
    sparse_values: None,
    metadata: None,
}, Vector {
    id: "id2".to_string(),
    values: vec1![2.0, 3.0, 4.0, 5.0],
    sparse_values: None,
    metadata: None,
}];
index.upsert(&vectors, &"namespace".into()).await.unwrap();
```

## Query index
### Query by index
```rust
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::pinecone::data::Namespace;

let pinecone = PineconeClient::new(None, None, None, None).unwrap();

// Connect to index host url
let mut index = pinecone.index("index-host").await.unwrap();

// Query the vector with id "vector-id" in the namespace "namespace"
index.query_by_id(
    "vector-id".to_string(),
    10,
    &Namespace::default(),
    None,
    None,
    None
).await.unwrap();
```

### Query by value
```rust
let pinecone = PineconeClient::new(None, None, None, None).unwrap();

let mut index = pinecone.index("index-host").await.unwrap();

let vector = vec![1.0, 2.0, 3.0, 4.0];

index.query_by_value(
    vector,
    None,
    10,
    &Namespace::default(),
    None,
    None,
    None
).await.unwrap();
```

## Delete vectors
By ID:
```rust
use pinecone_sdk::pinecone::PineconeClient;

let pinecone = PineconeClient::new(None, None, None, None).unwrap();

let mut index = pinecone.index("index-host").await.unwrap();

let ids = ["vector-id"]
index.delete_by_id(&ids, &"namespace".into()).await.unwrap();
```

By filter:
```rust
use std::collections::BTreeMap;
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::pinecone::data::{Metadata, Value, Kind, Namespace};

let pinecone = PineconeClient::new(None, None, None, None).unwrap();

let mut fields = BTreeMap::new();
fields.insert("field".to_string(), Value { kind: Some(Kind::StringValue("value".to_string()))});
index.delete_by_filter(Metadata { fields }, &"namespace".into()).await.unwrap();
```

Delete all:
```rust
use pinecone_sdk::pinecone::PineconeClient;

let pinecone = PineconeClient::new(None, None, None, None).unwrap();

let mut index = pinecone.index("index-host").await.unwrap();

index.delete_all(&"namespace".into()).await.unwrap();
```

## Fetch vectors
```rust
use pinecone_sdk::pinecone::PineconeClient;

let pinecone = PineconeClient::new(None, None, None, None).unwrap();

let mut index = pinecone.index("index-host").await.unwrap();

index.fetch(vectors, &Default::default()).await.unwrap();
```

## Update vectors
```rust
use pinecone_sdk::pinecone::PineconeClient;

let pinecone = PineconeClient::new(None, None, None, None).unwrap();

let mut index = pinecone.index("index-host").await.unwrap();

index.update("vector-id", vec![1.0, 2.0, 3.0, 4.0], None, None, &"namespace".into()).await.unwrap();
```

## List vectors
```rust
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::pinecone::data::Namespace;

let pinecone = PineconeClient::new("index-host").await.unwrap();

let mut index = pinecone.index("index-host").await.unwrap();

index.list(&"namespace".into(), None, None, None).await.unwrap();
```

# Collections
## Create collection
```rust
use pinecone_sdk::pinecone::{PineconeClient, control::CollectionModel};

let pinecone = PineconeClient::new(None, None, None, None).unwrap();

let create_collection_response = pinecone.create_collection("collection-name", "index-name").await;
```

## List collections
```rust
use pinecone_sdk::pinecone::PineconeClient;

let pinecone = PineconeClient::new(None, None, None, None).unwrap();

let response = pinecone.list_collections().await;
```

## Describe collection
```rust
use pinecone_sdk::pinecone::PineconeClient;

let pinecone = PineconeClient::new(None, None, None, None).unwrap();

let response = pinecone.describe_collection("collection-name").await;
```

## Delete collection
```rust
use pinecone_sdk::pinecone::PineconeClient;

let pinecone = PineconeClient::new(None, None, None, None).unwrap();

let response = pinecone.delete_collection("collection-name").await;
```
