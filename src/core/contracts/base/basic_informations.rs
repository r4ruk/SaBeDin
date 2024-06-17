use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

#[derive(Serialize, Deserialize, Clone)]
pub struct CommandResponse {
    pub code: i16,
    pub response: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestPostBodyWrapper {
    pub idempotency_key: String,
    pub method: String,
    pub object: String,
    pub params: HashMap<String, String>,
    pub query_options: QueryOptions,
    pub requesting_user_id: String,
}

impl From<RequestPostBodyWrapper> for RequestPostBody {
    fn from(value: RequestPostBodyWrapper) -> Self {
        Self {
            idempotency_key: value.idempotency_key,
            method: value.method,
            object: value.object,
            params: value.params,
            query_options: value.query_options,
        }
    }
}


#[derive(Serialize, Deserialize, Debug )]
pub struct ResponseBody {
    pub body: String,
}


pub fn new_simple_post_body(method: String, object: Value) -> RequestPostBody {
    RequestPostBody {
        idempotency_key: "".to_string(),
        method,
        object: object.to_string(),
        params: Default::default(),
        query_options: Default::default(),
    }
}