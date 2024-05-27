use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::core::contracts::base::query_params::QueryOptions;

// The request post body representation which can be further sent to processing Services
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestPostBody {
    pub idempotency_key: String,
    pub method: String,
    pub object: String,
    pub params: HashMap<String, String>,
    pub query_options: QueryOptions
}

#[derive(Serialize, Deserialize, Debug )]
pub struct ResponseBody {
    pub body: String,
}