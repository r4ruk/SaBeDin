use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::core::contracts::builtin_types::custom_uuid;


// template definition for further work on real entities and services.
#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(with = "custom_uuid")]
    pub id: Uuid,
    pub name: String,
    pub password: String,
    pub email: String,
    pub age: usize
}