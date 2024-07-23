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

### Indexes

## Create Index

# Create serverless index
The following example creates a serverless in the `us-east-1` region of AWS. For more information on serverless and regional availability, see [Understanding indexes](https://docs.pinecone.io/guides/indexes/understanding-indexes#serverless-indexes)
```
use pinecone_sdk::pinecone::{PineconeClient, control::{Metric, Cloud, WaitPolicy, IndexModel}};
use pinecone_sdk::utils::errors::PineconeError;
use std::time::Duration;

let pinecone = PineconeClient::new('PINECONE_API_KEY', None, None, None).unwrap();
 
pinecone.create_serverless_index(
    "index-name", // Name of the index
    10, // Dimension of the vectors
    Metric::Cosine, // Distance metric
    Cloud::Aws, // Cloud provider
    "us-east-1", // Region
    WaitPolicy::NoWait // Timeout
).await;
```

# Create pod index

## List indexes

## Describe index

## Delete index

## Describe index statistics

## Upsert vectors

## Query index

## Delete vectors

## Fetch vectors

## Update vectors

## List vectors


Sample code for using indexes

## Collections

Sample code for using colle
