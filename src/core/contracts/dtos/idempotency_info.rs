use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use crate::core::contracts::base::builtin_types::custom_uuid;

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct IdempotencyObject {
    #[serde(with="custom_uuid")]
    pub user_id: Uuid,
    pub idempotency_key: String,
    pub response_status_code: i16,
    pub response_body: Value,
}

#[derive(Debug)]
pub enum IdempotencyEvents {
    CreateIdempotencyKey,
    UpdateIdempotencyKey
}

impl IdempotencyEvents {
    pub fn name(&self) -> String{
        format!("{:?}", self)
    }
}

impl FromStr for IdempotencyEvents {
    type Err = ();

    fn from_str(s: &str) -> Result<IdempotencyEvents, Self::Err> {
        match s.to_lowercase().as_str() {
            "CreateIdempotencyKey" => Ok(IdempotencyEvents::CreateIdempotencyKey),
            "UpdateIdempotencyKey" => Ok(IdempotencyEvents::UpdateIdempotencyKey),
            _ => Err(()),
        }
    }
}