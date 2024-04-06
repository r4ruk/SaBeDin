use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::core::contracts::builtin_types::{custom_datetime, custom_uuid};

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Article {
    #[serde(with = "custom_uuid")]
    pub id: Uuid,
    pub programming_key_name: String,
    pub title: String,
    pub contents: String,
    pub tags: String,
    #[serde(rename = "createdAt")]
    #[serde(with = "custom_datetime")]
    pub created_at: DateTime<Utc>,
}