use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::Row;
use uuid::Uuid;
use crate::core::contracts::base::builtin_types::custom_uuid;
use crate::core::contracts::base::builtin_types::custom_datetime;

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct User {
    #[serde(with = "custom_uuid")]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub verified: bool,
    #[serde(rename = "createdAt")]
    #[serde(with = "custom_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[serde(with = "custom_datetime")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RegisterUserData {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoginUserData {
    pub email: String,
    pub password: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct FilteredUser {
    #[serde(with = "custom_uuid")]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub verified: bool,
    #[serde(rename = "createdAt")]
    #[serde(with = "custom_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[serde(with = "custom_datetime")]
    pub updated_at: DateTime<Utc>,
}



/// Implement From<PgRow> for FilteredUser
impl From<PgRow> for FilteredUser {
    fn from(row: PgRow) -> Self {
        // Extract fields from the row and construct a FilteredUser instance
        FilteredUser {
            id: row.get("id"),  // Adjust field names and types as necessary
            name: row.get("name"),
            email: row.get("email"),
            password: row.get("password"),
            role: row.get("role"),
            verified: row.get("verified"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct LoginUserResponse {
    pub token: String,
}

impl IntoResponse for User {
    fn into_response(self) -> Response {
        // let formatted_created = format!("{}", self.created_at.unwrap().format("%d/%m/%Y %H:%M"));

        Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(json!(self).to_string()))
            .unwrap()
    }
}