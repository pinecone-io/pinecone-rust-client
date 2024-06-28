use crate::pinecone::PineconeClient;
use crate::utils::errors::PineconeError;

/// Generated protobuf module for data plane.
pub mod pb {
    tonic::include_proto!("_");
}

/// A client for interacting with a Pinecone index.
pub struct Index {
    /// The name of the index.
    name: String,
    // channel: tonic::transport::Channel, // ?
}

impl Index {
    pub fn upsert() -> Result<(), PineconeError> {
        println!("Upserting index");
        Ok(())
    }
}

impl PineconeClient {
    /// Target an index for data operations.
    ///
    /// ### Arguments
    /// * `name: &str` - The name of the index to target.
    ///
    /// ### Return
    /// * `Result<Index, PineconeError>` - A Pinecone index object.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// use pinecone_sdk::pinecone::PineconeClient;
    ///
    /// let pinecone = PineconeClient::new(None, None, None, None).unwrap();
    /// let index = pinecone.index("my-index").unwrap();
    /// ```
    pub fn index(&self, name: &str) -> Result<Index, PineconeError> {
        let index = Index {
            name: name.to_string(),
            // channel: self.channel.clone(),
        };

        Ok(index)
    }
}
