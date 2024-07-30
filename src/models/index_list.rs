use crate::models::IndexModel;
use crate::openapi::models::IndexList as OpenApiIndexList;
#[derive(Clone, Default, Debug, PartialEq)]
pub struct IndexList {
    pub indexes: Option<Vec<IndexModel>>,
}

impl From<OpenApiIndexList> for IndexList {
    fn from(index_list: OpenApiIndexList) -> Self {
        IndexList {
            indexes: index_list.indexes.map(|e| e.into_iter().map(|e| e.into()).collect()),
        }
    }
}
