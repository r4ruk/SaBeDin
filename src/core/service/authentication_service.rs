use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use axum::http::StatusCode;
use rand_core::OsRng;
use sqlx::Executor;
use crate::core::contracts::errors::GeneralServerError;
use crate::core::contracts::user::{LoginUserData, RegisterUserData};
use crate::core::persistence::db_pool::get_db_pool;

pub async fn register_user(user_data: RegisterUserData) -> Result<(), GeneralServerError> {

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(user_data.password.as_bytes(), &salt)
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Error while hashing password: {}", e),
            });
        })
        .map(|hash| hash.to_string()).map_err(|_| GeneralServerError{message: "error".to_string()});

    // TODO write the stuff to DB
    return Ok(());
}

/// function returns bool about state of the login attempt
pub async fn login(user_data:LoginUserData) -> bool {

    // TODO retrieve user from database and compare stuff
    let pool = match get_db_pool() {
        Some(pool) => pool,
        None => return false
    };



    let is_valid = match PasswordHash::new(&user_data.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(user_data.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };
    return is_valid
}