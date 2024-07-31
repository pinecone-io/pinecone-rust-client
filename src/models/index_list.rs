use super::IndexModel;
use crate::openapi::models::IndexList as OpenApiIndexList;

/// IndexList : The list of indexes that exist in the project.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct IndexList {
    /// The list of indexes
    pub indexes: Option<Vec<IndexModel>>,
}

impl From<OpenApiIndexList> for IndexList {
    fn from(index_list: OpenApiIndexList) -> Self {
        IndexList {
            indexes: index_list
                .indexes
                .map(|index| index.into_iter().map(|index| index.into()).collect()),
        }
    }
}
