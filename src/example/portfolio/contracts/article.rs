use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;
use uuid::Uuid;
use crate::core::contracts::base::builtin_types::{custom_datetime, custom_uuid};

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Article {
    #[serde(with = "custom_uuid")]
    pub id: Uuid,
    pub programming_key_name: String,
    pub title: String,
    pub contents: String,
    pub tags: String,
    #[serde(with = "custom_datetime")]
    pub created_at: DateTime<Utc>,
}

/// Implement From<PgRow> for Article
impl From<PgRow> for Article {
    fn from(row: PgRow) -> Self {
        // Extract fields from the row and construct an Article instance
        Article {
            id: row.get("id"),  // Adjust field names and types as necessary
            programming_key_name: row.get("programming_key_name"),
            title: row.get("title"),
            contents: row.get("contents"),
            tags: row.get("tags"),
            created_at: row.get("created_at"),
        }
    }
}