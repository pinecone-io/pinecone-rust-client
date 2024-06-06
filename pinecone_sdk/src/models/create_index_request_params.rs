
pub struct CreateServerlessIndexRequest {
    pub name: String,
    pub dimension: i32,
    pub metric: Option<String>,
    pub cloud: Option<String>,
    pub region: String,
}

impl CreateServerlessIndexRequest {
    pub fn new(name: &str, dimension: i32, metric: Option<&str>, cloud: Option<&str>, region: &str) -> CreateServerlessIndexRequest {
        CreateServerlessIndexRequest {
            name: name.to_string(),
            dimension,
            metric: if let Some(metric) = metric { Some(metric.to_string()) } else { None },
            cloud: if let Some(cloud) = cloud { Some(cloud.to_string()) } else { None },
            region: region.to_string(),
        }
    }
}