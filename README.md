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

## Upsert vectors

```rust
use pinecone_sdk::pinecone::PineconeClient;
use pinecone_sdk::pinecone::data::Vector;

let pinecone = PineconeClient::new(None, None, None, None).unwrap();

let mut index = pinecone.index("index-host").await.unwrap();

let vectors = vec![Vector {
    id: "vector-id".to_string(),
    values: vec![1.0, 2.0, 3.0, 4.0],
    sparse_values: None,
    metadata: None,
}];
index.upsert(vectors, None).await.unwrap();
```

## Query index

## Delete vectors

## Fetch vectors

## Update vectors

## List vectors

# Collections
