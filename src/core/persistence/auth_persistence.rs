use std::iter::Filter;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use uuid::Uuid;
use crate::core::contracts::errors::GeneralServerError;
use crate::core::contracts::user::{FilteredUser, LoginUserData, User};
use crate::core::persistence::db_pool::get_db_pool;

pub async fn login_user(user_data: LoginUserData) -> Result<FilteredUser, GeneralServerError> {
    let pool = match get_db_pool() {
        Some(pool) => pool,
        None => {
            println!("could not retrieve pool");
            return Err(GeneralServerError{ message: "no dbpool connection found".to_string() })
        }
    };
    let user = sqlx::query_as!(
        FilteredUser,
        "SELECT * FROM users WHERE email = $1",
        user_data.email.to_ascii_lowercase()
    )
        .fetch_optional(&pool.pool)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: {}", e),
        });
            GeneralServerError{message: error_response.to_string() }
        })?
        .ok_or_else(|| {
            let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid email or password",
        });
            GeneralServerError{message: error_response.to_string() }
        })?;

    return Ok(user)
}