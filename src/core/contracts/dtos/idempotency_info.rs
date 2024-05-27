use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use crate::core::contracts::base::builtin_types::custom_uuid;

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct IdempotencyObject {
    #[serde(with="custom_uuid")]
   pub user_id: Uuid,
   pub idempotency_key: String,
   pub response_status_code: i8,
   pub response_body: Value,
}