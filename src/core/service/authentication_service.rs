use std::future::Future;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use axum::http::StatusCode;
use rand_core::OsRng;
use sqlx::Executor;
use crate::core::contracts::errors::GeneralServerError;
use crate::core::contracts::user::{FilteredUser, LoginUserData, RegisterUserData};
use crate::core::persistence::auth_persistence;
use crate::core::persistence::db_pool::get_db_pool;
use crate::core::service::authentication_service;

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

    let mut cloned_user_data = user_data.clone();
    match hashed_password {
        Ok(hashed) => {cloned_user_data.password = hashed}
        Err(e) => {return Err(e)}
    }

    let registered_user = auth_persistence::register_user(cloned_user_data).await;
    match registered_user {
        Ok(_) => (),
        Err(e) => {
            return Err(e)
        }
    }

    return Ok(());
}

/// function returns bool about state of the login attempt
pub async fn login(user_data:LoginUserData) -> bool {

    let dbuser = auth_persistence::login_user(user_data.clone()).await;

    let user: FilteredUser = match dbuser {
        Ok(u) => u,
        _ => return false
    };

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(user_data.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };
    return is_valid
}