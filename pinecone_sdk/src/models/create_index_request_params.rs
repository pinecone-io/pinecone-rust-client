
#[derive(Debug)]
pub struct CreateIndexParams {
    pub name: String,
    pub dimension: u32,
    pub metric: Metric,
    pub spec: Spec,
}

impl CreateIndexParams {
    pub fn new(name: &str, dimension: u32, metric: Option<Metric>, spec: Spec) -> CreateIndexParams {
        CreateIndexParams {
            name: name.to_string(),
            dimension,
            metric: metric.map(|x| x).unwrap_or(Metric::Cosine),
            spec,
        }
    }
}

#[derive(Debug)]
pub enum Spec {
    Serverless {
        cloud: Cloud,
        region: String,
    },
    Pod {
        environment: String,
        replicas: Option<i32>,
        shards: Option<i32>,
        pod_type: String,
        pods: i32,
        metadata_config: Option<String>,
        source_collection: Option<String>,
    },
}

#[derive(Debug)]
pub enum Metric {
    Cosine,
    Euclidean,
    Dotproduct,
}

#[derive(Debug)]
pub enum Cloud {
    Aws,
    Gcp,
    Azure,
}
