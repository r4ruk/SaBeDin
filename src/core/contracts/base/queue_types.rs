use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::core::contracts::base::basic_informations::RequestPostBody;
use crate::core::contracts::base::builtin_types::{custom_uuid, custom_datetime};


#[derive(Serialize, Deserialize)]
pub struct QueueRequestMessage {
    #[serde(with = "custom_uuid")]
    pub message_id: Uuid,
    #[serde(with = "custom_uuid")]
    pub correlation_id: Uuid,
    pub headers: String,
    pub body: RequestPostBody,
    #[serde(with = "custom_datetime")]
    pub timestamp: DateTime<Utc>,
}

// body should be the expected services deserializable object structure representation in JSON
pub struct QueueResponseMessage {
    pub correlation_id: Uuid,
    pub body: String
}