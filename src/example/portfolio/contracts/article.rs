use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;
use uuid::Uuid;
use crate::core::contracts::builtin_types::{custom_datetime, custom_uuid};
use crate::core::contracts::user::FilteredUser;

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

/// Implement From<PgRow> for FilteredUser
impl From<PgRow> for Article {
    fn from(row: PgRow) -> Self {
        // Extract fields from the row and construct a FilteredUser instance
        // TODO add remaining information
        Article {
            id: row.get("id"),  // Adjust field names and types as necessary
            programming_key_name: "".to_string(),
            title: "".to_string(),
            contents: "".to_string(),
            tags: "".to_string(),
            created_at: Default::default(),
        }
    }
}