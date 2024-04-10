use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// The request post body representation which can be further sent to processing Services
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPostBody {
    pub method: String,
    pub object: String,
    pub params: HashMap<String, String>,
    pub paging_query: PagingQuery
}

#[derive(Serialize, Deserialize, Debug )]
pub struct ResponseBody {
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PagingQuery {
    pub amount: i16,
    pub page_num: i16
}
