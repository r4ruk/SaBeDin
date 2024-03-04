use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString};
use rand_core::OsRng;
use serde_json::Value;

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
