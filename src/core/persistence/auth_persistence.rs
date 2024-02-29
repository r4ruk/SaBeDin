use crate::core::contracts::dependency_container::ExecutionContext;
use crate::core::contracts::errors::GeneralServerError;
use crate::core::contracts::user::{FilteredUser, LoginUserData, RegisterUserData};
use crate::core::persistence::persistence_utils;

pub async fn login_user(context: &ExecutionContext, user_data: LoginUserData) -> Result<FilteredUser, GeneralServerError> {
    let user = sqlx::query_as!(
        FilteredUser,
        "SELECT * FROM users WHERE email = $1",
        user_data.email.to_ascii_lowercase()
    )
        .fetch_optional(&context.db)
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

pub async fn register_user(context: &ExecutionContext, user_data: RegisterUserData) -> Result<FilteredUser, GeneralServerError>{

    let user_exists: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(user_data.email.to_owned().to_ascii_lowercase())
            .fetch_one(&context.db)
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
        .fetch_one(&context.db)
        .await
        .map_err(|e| {
            GeneralServerError{message: persistence_utils::map_to_error_response(e)}
        })?;

    return Ok(user)
}

