use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::core::persistence::query_builder::{QueryClause, Sorting};

// The request post body representation which can be further sent to processing Services
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPostBody {
    pub method: String,
    pub object: String,
    pub params: HashMap<String, String>,
    pub query_options: QueryOptions
}

#[derive(Serialize, Deserialize, Debug )]
pub struct ResponseBody {
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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