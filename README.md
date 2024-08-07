# Pinecone Rust SDK

This SDK is still in an alpha state. While it is mostly built out and functional, it may undergo changes as we continue to improve the UX. Please try it out and give us your feedback, but be aware that updates may introduce breaking changes.

## Documentation

The documentation can be found [here](https://docs.rs/pinecone-sdk/latest/pinecone_sdk/).

## Prerequisites

Rust version: tested with Rust version 1.78.0

Before you can use the Pinecone SDK, you must sign up for an account and find your API key in the Pinecone console dashboard at [https://app.pinecone.io](https://app.pinecone.io).

## Installation

How to install - instruction for getting the package from crates.io

## Usage

The `PineconeClient` class is the main point of entry into the Rust SDK. Parameters may either be directly passed in as `Options`, or read through environment variables as follows. More detail:
- The API key must be passed in either as an argument or as an environment variable called `PINECONE_API_KEY`. If passed in as `None`, the client will attempt to read in an environment variable value.
- The control plane host, if passed in as `None`, will attempt to read in an environment variable called `PINECONE_CONTROLLER_HOST`. If it is not an environment variable, it will default to `https://api.pinecone.io`.

There are a couple of ways to instantiate the client, detailed below:
### PineconeClientConfig
Initialize a `PineconeClientConfig` struct with parameters, and call `config` using the struct.
```rust
use pinecone_sdk::pinecone::{PineconeClient, PineconeClientConfig};

let config = PineconeClientConfig {
    api_key: Some("INSERT_API_KEY".to_string()),
    control_plane_host: Some("INSERT_CONTROLLER_HOST".to_string()),
    ..Default::default()
};

let pinecone: PineconeClient = config.client().expect("Failed to create Pinecone instance");
```

### Default client
Use the `default_client()` function, which is the equivalent of constructing a `PineconeClientConfig` struct with all fields set to `None`. The API key and control plane host (optional) will be read from environment variables.
```rust
let pinecone: PineconeClient = pinecone_sdk::pinecone::default_client().expect("Failed to create Pinecone instance");
```

# Indexes

## Create Index

### Create serverless index
The following example creates a serverless index in the `us-east-1` region of AWS. For more information on serverless and regional availability, see [Understanding indexes](https://docs.pinecone.io/guides/indexes/understanding-indexes#serverless-indexes)
```rust
use pinecone_sdk::pinecone::PineconeClientConfig;
use pinecone_sdk::pinecone::control::{Metric, Cloud, WaitPolicy, IndexModel};

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;
 
let index_description: IndexModel = pinecone.create_serverless_index(
    "index-name",       // Name of the index
    10,                 // Dimension of the vectors
    Metric::Cosine,     // Distance metric
    Cloud::Aws,         // Cloud provider
    "us-east-1",        // Region
    WaitPolicy::NoWait  // Timeout
).await?;
```

### Create pod index
The following example creates a pod index in the `us-east-1` region of AWS.
```rust
use pinecone_sdk::pinecone::PineconeClientConfig;
use pinecone_sdk::pinecone::control::{Metric, Cloud, WaitPolicy, IndexModel};

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

let index_description: IndexModel = pinecone.create_pod_index(
    "index-name",       // Index name
    10,                 // Dimension
    Metric::Cosine,     // Distance metric
    "us-east-1",        // Region
    "p1.x1",            // Pod type
    1,                  // Number of pods
    None,               // Number of replicas
    None,               // Number of shards
    None,               // Metadata to index
    None,               // Source collection
    WaitPolicy::NoWait  // Wait policy
).await?;
```

Pod indexes support several optional configuration fields. The following example constructs a pod index with some specification for these fields.
```rust
use pinecone_sdk::pinecone::PineconeClientConfig;
use pinecone_sdk::pinecone::control::{Metric, Cloud, WaitPolicy, IndexModel};

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

let index_description: IndexModel = pinecone.create_pod_index(
    "index-name",       // Index name
    10,                 // Dimension
    Metric::Cosine,     // Distance metric
    "us-east-1",        // Region
    "p1.x1",            // Pod type
    1,                  // Number of pods
    Some(1),            // Number of replicas
    Some(1),            // Number of shards
    Some(               // Metadata fields to index
        &vec!["genre",
            "title",
            "imdb_rating"]),
    Some("collection"), // Source collection
    WaitPolicy::NoWait  // Wait policy
).await?;
```

## List indexes
The following example lists all indexes in your project.
```rust
use pinecone_sdk::pinecone::{PineconeClientConfig, control::IndexList};

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

let index_list: IndexList = pinecone.list_indexes().await?;
```

## Describe index
The following example returns information about the index `index-name`.
```rust
use pinecone_sdk::pinecone::{PineconeClientConfig, control::IndexModel};

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

let index_description: IndexModel = pinecone.describe_index("index-name").await?;
```

## Configure index
Configuring an index takes in three optional parameters -- a DeletionProtection enum, the number of replicas, and the pod type. The deletion protection can be updated for any index type, while the number of replicas and the pod type can only be updated for pod indexes.

The following example disables deletion protection for the index `index-name`.
```rust
use pinecone_sdk::pinecone::{PineconeClientConfig, control::IndexModel};

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

let index_description: IndexModel = pinecone.configure_index("index-name", Some(DeletionProtection::Disabled), None, None).await?;
```

The following example changes the index `index-name` to have 6 replicas and pod type `s1`. The deletion protection type will not be changed in this case.
```rust
use pinecone_sdk::pinecone::{PineconeClientConfig, control::IndexModel};

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;
    
let index_description: IndexModel = pinecone.configure_index("index-name", None, Some(6), Some("s1")).await?;
```

## Delete index
The following example deletes the index `index-name`.
```rust
use pinecone_sdk::pinecone::PineconeClientConfig;

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;
    
pinecone.delete_index("index-name").await?;
```

## Describe index statistics
The following example returns statistics about the index with host `index-host`.
Without filter
```rust
use std::collections::BTreeMap;
use pinecone_sdk::pinecone::PineconeClientConfig;
use pinecone_sdk::pinecone::data::DescribeIndexStatsResponse;

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

let mut index = pinecone.index("index-host").await?;

let response: DescribeIndexStatsResponse = index.describe_index_stats(None).await?;
```

With filter
```rust
use std::collections::BTreeMap;
use pinecone_sdk::pinecone::PineconeClientConfig;
use pinecone_sdk::pinecone::data::{Value, Kind, Metadata, DescribeIndexStatsResponse};

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

let mut index = pinecone.index("index-host").await?;

let mut fields = BTreeMap::new();
let kind = Some(Kind::StringValue("value".to_string()));
fields.insert("field".to_string(), Value { kind });

let response: DescribeIndexStatsResponse = index.describe_index_stats(Some(Metadata { fields })).await?;
```

## Upsert vectors
The following example upserts two vectors into the index with host `index-host`.
```rust
use pinecone_sdk::pinecone::PineconeClientConfig;
use pinecone_sdk::pinecone::data::{Vector, UpsertResponse};

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

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

## Query vectors
There are two supported ways of querying an index.
### Query by index
The following example queries the index with host `index-host` for the vector with ID `vector-id`, and returns the top 10 matches.
```rust
use pinecone_sdk::pinecone::PineconeClientConfig;
use pinecone_sdk::pinecone::data::{Namespace, QueryResponse};

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

// Connect to index at host "index-host"
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
The following example queries the index with host `index-host` for the vector with values `[1.0, 2.0, 3.0, 4.0]`, and returns the top 10 matches.
```rust
use pinecone_sdk::pinecone::PineconeClientConfig;
use pinecone_sdk::pinecone::data::{Namespace, QueryResponse};

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

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
There are three supported ways of deleting vectors.
### Delete by ID
The following example deletes the vector with ID `vector-id` in the namespace `namespace`.
```rust
use pinecone_sdk::pinecone::PineconeClientConfig;

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

let mut index = pinecone.index("index-host").await?;

let ids = ["vector-id"]

index.delete_by_id(&ids, &"namespace".into()).await?;
```

### Delete by filter:
The following example deletes vectors that satisfy the filter in the namespace `namespace`.
```rust
use std::collections::BTreeMap;
use pinecone_sdk::pinecone::PineconeClientConfig;
use pinecone_sdk::pinecone::data::{Metadata, Value, Kind, Namespace};

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

let mut fields = BTreeMap::new();
let kind = Some(Kind::StringValue("value".to_string()));
fields.insert("field".to_string(), Value { kind });

index.delete_by_filter(Metadata { fields }, &"namespace".into()).await?;
```

### Delete all:
The following example deletes all vectors in the namespace `namespace`.
```rust
use pinecone_sdk::pinecone::PineconeClientConfig;

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

let mut index = pinecone.index("index-host").await?;

index.delete_all(&"namespace".into()).await?;
```

## Fetch vectors
The following example fetches the vectors with IDs `1` and `2` from the default namespace.
```rust
use pinecone_sdk::pinecone::PineconeClientConfig;
use pinecone_sdk::pinecone::data::FetchResponse;

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

let mut index = pinecone.index("index-host").await?;

let vectors = &["1", "2"];

let response: FetchResponse = index.fetch(vectors, &Default::default()).await?;
```

## Update vectors
The following example updates the vector with ID `vector-id` in the namespace `namespace` to have values `[1.0, 2.0, 3.0, 4.0]`.
```rust
use pinecone_sdk::pinecone::PineconeClientConfig;
use pinecone_sdk::pinecone::data::UpdateResponse;

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

let mut index = pinecone.index("index-host").await?;

let response: UpdateResponse = index.update("vector-id", vec![1.0, 2.0, 3.0, 4.0], None, None, &"namespace".into()).await?;
```

## List vectors
The following example lists vectors in the namespace `namespace`.
```rust
use pinecone_sdk::pinecone::PineconeClientConfig;
use pinecone_sdk::pinecone::data::{Namespace, ListResponse};

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

let mut index = pinecone.index("index-host").await?;

let response: ListResponse = index.list(&"namespace".into(), None, None, None).await?;
```

# Collections
## Create collection
The following example creates a collection `collection-name` in the index `index-name`.
```rust
use pinecone_sdk::pinecone::PineconeClientConfig;
use pinecone_sdk::pinecone::control::CollectionModel;

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

let collection: CollectionModel = pinecone.create_collection("collection-name", "index-name").await?;
```

## List collections
The following example lists all collections in a project.
```rust
use pinecone_sdk::pinecone::PineconeClientConfig;
use pinecone_sdk::pinecone::control::CollectionList;

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

let collection_list: CollectionList = pinecone.list_collections().await?;
```

## Describe collection
The following example describes the collection `collection-name`.
```rust
use pinecone_sdk::pinecone::PineconeClientConfig;
use pinecone_sdk::pinecone::control::CollectionModel;

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

let collection: CollectionModel = pinecone.describe_collection("collection-name").await?;
```

## Delete collection
The following example deletes the collection `collection-name`.
```rust
use pinecone_sdk::pinecone::PineconeClientConfig;

let config = PineconeClientConfig {
    api_key: Some('<<PINECONE_API_KEY>>'),
    ..Default::default()
};
let pinecone = config.client()?;

pinecone.delete_collection("collection-name").await?;
```

# Contributing
If you'd like to make a contribution, or get setup locally to develop the Pinecone Rust client, please see our [contributing guide](https://github.com/pinecone-io/pinecone-rust-client/blob/emily/update-readme/CONTRIBUTING.md)
