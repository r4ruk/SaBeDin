use std::iter::Filter;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use log::Level::Error;
use uuid::Uuid;
use crate::core::contracts::errors::GeneralServerError;
use crate::core::contracts::user::{FilteredUser, LoginUserData, RegisterUserData, User};
use crate::core::persistence::db_pool::get_db_pool;
use crate::core::persistence::persistence_utils;

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
            GeneralServerError{message: persistence_utils::map_to_error_response(e)}
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

pub async fn register_user(user_data: RegisterUserData) -> Result<FilteredUser, GeneralServerError>{
    let pool = match get_db_pool() {
        Some(pool) => pool,
        None => {
            println!("could not retrieve pool");
            return Err(GeneralServerError{ message: "no dbpool connection found".to_string() })
        }
    };

    let user_exists: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(user_data.email.to_owned().to_ascii_lowercase())
            .fetch_one(&pool.pool)
            .await
            .map_err(|e| {
                GeneralServerError{message: persistence_utils::map_to_error_response(e)}
            })?;

    match user_exists {
        Some(exists_state) => {
            if exists_state {
                return Err(GeneralServerError{message: "user already exists".to_string()})
            }
        }
        None => (),
    }

    let user = sqlx::query_as!(
            FilteredUser,
            "INSERT INTO users (name,email,password) VALUES ($1, $2, $3) RETURNING *",
            user_data.name.to_string(),
            user_data.email.to_string().to_ascii_lowercase(),
            user_data.password.to_string()
        )
        .fetch_one(&pool.pool)
        .await
        .map_err(|e| {
            GeneralServerError{message: persistence_utils::map_to_error_response(e)}
        })?;

    return Ok(user)
}

