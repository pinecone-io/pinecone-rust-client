use crate::pinecone::PineconeClient;

pub mod pb {
    tonic::include_proto!("_");
}

impl PineconeClient {
    pub fn foo() {
        let req = pb::RequestUnion::default();
    }
}
