use crate::openapi::models::create_index_request::Metric as RequestMetric;
use crate::openapi::models::index_model::Metric as ResponseMetric;

/// The distance metric to be used for similarity search. You can use 'euclidean', 'cosine', or 'dotproduct'.
#[derive(Clone, Default, Debug, PartialEq)]
pub enum Metric {
    /// Cosine similarity
    #[default]
    Cosine,
    /// Euclidean distance similarity
    Euclidean,
    /// Dot product similarity
    Dotproduct,
}

impl From<RequestMetric> for Metric {
    fn from(openapi_model: RequestMetric) -> Self {
        match openapi_model {
            RequestMetric::Cosine => Metric::Cosine,
            RequestMetric::Euclidean => Metric::Euclidean,
            RequestMetric::Dotproduct => Metric::Dotproduct,
        }
    }
}

impl From<ResponseMetric> for Metric {
    fn from(openapi_model: ResponseMetric) -> Self {
        match openapi_model {
            ResponseMetric::Cosine => Metric::Cosine,
            ResponseMetric::Euclidean => Metric::Euclidean,
            ResponseMetric::Dotproduct => Metric::Dotproduct,
        }
    }
}

impl From<Metric> for RequestMetric {
    fn from(model: Metric) -> Self {
        match model {
            Metric::Cosine => RequestMetric::Cosine,
            Metric::Euclidean => RequestMetric::Euclidean,
            Metric::Dotproduct => RequestMetric::Dotproduct,
        }
    }
}

impl From<Metric> for ResponseMetric {
    fn from(model: Metric) -> Self {
        match model {
            Metric::Cosine => ResponseMetric::Cosine,
            Metric::Euclidean => ResponseMetric::Euclidean,
            Metric::Dotproduct => ResponseMetric::Dotproduct,
        }
    }
}
