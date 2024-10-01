use pinecone_sdk::models::Namespace;
use rand::Rng;

/// Generates a random string of length 10
pub fn generate_random_string() -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let s: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    s.to_lowercase()
}

/// Generates a random index name
#[allow(dead_code)]
pub fn generate_index_name() -> String {
    format!("test-index-{}", generate_random_string())
}

/// Generates a random collection name
#[allow(dead_code)]
pub fn generate_collection_name() -> String {
    format!("test-collection-{}", generate_random_string())
}

/// Generates a random namespace name
#[allow(dead_code)]
pub fn generate_namespace_name() -> Namespace {
    let name = format!("test-namespace-{}", generate_random_string());
    name.into()
}

/// Generates a random vector of length `length`
#[allow(dead_code)]
pub fn generate_vector(length: usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen()).collect()
}

/// Returns the name of the serverless index from the environment variable
pub fn get_serverless_index() -> String {
    std::env::var("SERVERLESS_INDEX_NAME").unwrap()
}

/// Returns the name of the pod collection from the environment variable
pub fn get_pod_index() -> String {
    std::env::var("POD_INDEX_NAME").unwrap()
}

/// Returns the name of the collection from the environment variable
#[allow(dead_code)]
pub fn get_collection() -> String {
    std::env::var("COLLECTION_NAME").unwrap()
}
