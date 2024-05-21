use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::core::contracts::query_params::QueryOptions;
use crate::core::contracts::builtin_types::custom_uuid;

// The request post body representation which can be further sent to processing Services
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPostBody {
    #[serde(with = "custom_uuid")]
    pub idempotency_key: Uuid,
    pub method: String,
    pub object: String,
    pub params: HashMap<String, String>,
    pub query_options: QueryOptions
}

#[derive(Serialize, Deserialize, Debug )]
pub struct ResponseBody {
    pub body: String,
}