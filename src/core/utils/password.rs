use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString};
use rand_core::OsRng;
use serde_json::Value;
use crate::core::contracts::dtos::user::FilteredUser;

pub fn hash_password(pwd: &str) -> Result<String, Value>{
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(pwd.as_bytes(), &salt)
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Error while hashing password: {}", e),
            });
            return error_response
        })
        .map(|hash| hash.to_string())
}

pub fn check_password_hash(given_password: String, stored_user: FilteredUser) -> bool {
    let hash = PasswordHash::new(&stored_user.password);
    let is_valid = match hash {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(given_password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };
    return is_valid
}