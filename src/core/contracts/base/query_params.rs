use serde::{Deserialize, Serialize};
use crate::core::persistence::core::query_builder::{QueryClause, Sorting};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PagingQuery {
    pub amount_of_items: i16,
    pub page_num: i16
}

impl Default for PagingQuery {
    fn default() -> Self {
        return Self {
            amount_of_items: 20,
            page_num: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryOptions {
    pub queries: Vec<QueryClause>,
    pub paging_information: PagingQuery,
    pub sorting_information: Sorting,
}

impl Default for QueryOptions {
    fn default() -> Self {
        Self {
            queries: vec![],
            paging_information: Default::default(),
            sorting_information: Sorting::Default,
        }
    }
}
