use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub code: i32,
    pub message: String,
    pub redirect: String
}