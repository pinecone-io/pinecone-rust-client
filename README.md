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
use pinecone_sdk::pinecone::{PineconeClient, control::{Metric, Cloud, WaitPolicy, IndexModel}};

let pinecone = PineconeClient::new('<<PINECONE_API_KEY>>', None, None, None)?;
 
let response: IndexModel = pinecone.create_serverless_index(
    "index-name",       // Name of the index
    10,                 // Dimension of the vectors
    Metric::Cosine,     // Distance metric
    Cloud::Aws,         // Cloud provider
    "us-east-1",        // Region
    WaitPolicy::NoWait  // Timeout
).await?;
```

### Create pod index
The following example creates a pod index in the `us-east-1` region of AWS. This example does not create replicas, or shards, nor use metadata or a source collection.
```rust
use pinecone_sdk::pinecone::{PineconeClient, control::{Metric, Cloud, WaitPolicy, IndexModel}};

let pinecone = PineconeClient::new('<<PINECONE_API_KEY>>', None, None, None)?;

let response: IndexModel = pinecone.create_pod_index(
    "index-name",       // index name
    10,                 // dimension
    Metric::Cosine,     // distance metric
    "us-east-1",        // region
    "p1.x1",            // pod type
    1,                  // number of pods
    None,               // number of replicas
    None,               // number of shards
    None,               // metadata
    None,               // source collection
    WaitPolicy::NoWait  // wait policy
).await?;
```

## List indexes
```rust
use pinecone_sdk::pinecone::{ClientClient, control::IndexList};

let pinecone = PineconeClient::new('<<PINECONE_API_KEY>>', None, None, None)?;

let response: IndexList = pinecone.list_indexes().await?;
```

## Describe index
```rust
use pinecone_sdk::pinecone::{PineconeClient, control::IndexModel};

let pinecone = PineconeClient::new('<<PINECONE_API_KEY>>', None, None, None)?;

let response: IndexModel = pinecone.describe_index("index-name").await?;
```

## Configure index
```rust
use pinecone_sdk::pinecone::{PineconeClient, control::IndexModel};

let pinecone = PineconeClient::new(None, None, None, None)?;
    
let response: IndexModel = pinecone.configure_index("index-name", 6, "s1").await?;
```

## Delete index
```rust
use pinecone_sdk::pinecone::PineconeClient;

let pinecone = PineconeClient::new(None, None, None, None)?;
    
pinecone.delete_index("index-name").await?;
```

## Describe index statistics
Without filter
```rust
use std::collections::BTreeMap;
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::pinecone::data::DescribeIndexStatsResponse;

let pinecone = PineconeClient::new(None, None, None, None)?;

let mut index = pinecone.index("index-host").await?;

let response: DescribeIndexStatsResponse = index.describe_index_stats(None).await?;
```

With filter
```rust
use std::collections::BTreeMap;
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::pinecone::data::{Value, Kind, Metadata, DescribeIndexStatsResponse};

let pinecone = PineconeClient::new(None, None, None, None)?;

let mut index = pinecone.index("index-host").await?;

let mut fields = BTreeMap::new();
fields.insert("field".to_string(), Value { kind: Some(Kind::StringValue("value".to_string()))});

let response: DescribeIndexStatsResponse = index.describe_index_stats(Some(Metadata { fields })).await?;
```

## Upsert vectors
```rust
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::pinecone::data::{Vector, UpsertResponse};

let pinecone = PineconeClient::new(None, None, None, None)?;

let mut index = pinecone.index("index-host").await?;

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

let response: UpsertResponse = index.upsert(&vectors, &"namespace".into()).await?;
```

## Query index
### Query by index
```rust
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::pinecone::data::{Namespace, QueryResponse};

let pinecone = PineconeClient::new(None, None, None, None)?;

// Connect to index host url
let mut index = pinecone.index("index-host").await?;

// Query the vector with id "vector-id" in the namespace "namespace"
let response: QueryResponse = index.query_by_id(
    "vector-id".to_string(),
    10,
    &Namespace::default(),
    None,
    None,
    None
).await?;
```

### Query by value
```rust
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::pinecone::data::{Namespace, QueryResponse};

let pinecone = PineconeClient::new(None, None, None, None)?;

let mut index = pinecone.index("index-host").await?;

let vector = vec![1.0, 2.0, 3.0, 4.0];

let response: QueryResponse = index.query_by_value(
    vector,
    None,
    10,
    &Namespace::default(),
    None,
    None,
    None
).await?;
```

## Delete vectors
By ID:
```rust
use pinecone_sdk::pinecone::PineconeClient;

let pinecone = PineconeClient::new(None, None, None, None)?;

let mut index = pinecone.index("index-host").await?;

let ids = ["vector-id"]

index.delete_by_id(&ids, &"namespace".into()).await?;
```

By filter:
```rust
use std::collections::BTreeMap;
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::pinecone::data::{Metadata, Value, Kind, Namespace};

let pinecone = PineconeClient::new(None, None, None, None)?;

let mut fields = BTreeMap::new();
fields.insert("field".to_string(), Value { kind: Some(Kind::StringValue("value".to_string()))});

index.delete_by_filter(Metadata { fields }, &"namespace".into()).await?;
```

Delete all:
```rust
use pinecone_sdk::pinecone::PineconeClient;

let pinecone = PineconeClient::new(None, None, None, None)?;

let mut index = pinecone.index("index-host").await?;

index.delete_all(&"namespace".into()).await?;
```

## Fetch vectors
```rust
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::pinecone::data::FetchResponse;

let pinecone = PineconeClient::new(None, None, None, None)?;

let mut index = pinecone.index("index-host").await?;

let response: FetchResponse = index.fetch(vectors, &Default::default()).await?;
```

## Update vectors
```rust
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::pinecone::data::UpdateResponse;

let pinecone = PineconeClient::new(None, None, None, None)?;

let mut index = pinecone.index("index-host").await?;

let response: UpdateResponse = index.update("vector-id", vec![1.0, 2.0, 3.0, 4.0], None, None, &"namespace".into()).await?;
```

## List vectors
```rust
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::pinecone::data::{Namespace, ListResponse};

let pinecone = PineconeClient::new("index-host").await?;

let mut index = pinecone.index("index-host").await?;

let response: ListResponse = index.list(&"namespace".into(), None, None, None).await?;
```

# Collections
## Create collection
```rust
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::pinecone::control::CollectionModel;

let pinecone = PineconeClient::new(None, None, None, None)?;

let response: CollectionModel = pinecone.create_collection("collection-name", "index-name").await?;
```

## List collections
```rust
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::pinecone::control::CollectionList;

let pinecone = PineconeClient::new(None, None, None, None)?;

let response: CollectionList = pinecone.list_collections().await?;
```

## Describe collection
```rust
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::pinecone::control::CollectionModel;

let pinecone = PineconeClient::new(None, None, None, None)?;

let response: CollectionModel = pinecone.describe_collection("collection-name").await?;
```

## Delete collection
```rust
use pinecone_sdk::pinecone::PineconeClient;

let pinecone = PineconeClient::new(None, None, None, None)?;

pinecone.delete_collection("collection-name").await?;
```
