use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use crate::core::contracts::dependency_container::ExecutionContext;
use crate::core::contracts::errors::GeneralServerError;
use crate::core::contracts::user::{FilteredUser, LoginUserData, RegisterUserData};
use crate::core::persistence::auth_persistence;
use crate::core::service::service_base::handle_finish_transaction;
use crate::core::utils::password::{check_password_hash, hash_password};

/// function registers user if he/she doesn't exist yet
pub async fn register_user(context: &ExecutionContext, user_data: RegisterUserData) -> Result<(), GeneralServerError> {
    let hashed_password = hash_password(&user_data.password)
        .map_err(|e| GeneralServerError{message: e.to_string()});

    let mut cloned_user_data = user_data.clone();
    match hashed_password {
        Ok(hashed) => {cloned_user_data.password = hashed}
        Err(e) => {return Err(e)}
    }

    let user_exists = check_user_exists(context, user_data.email.clone()).await;
    if user_exists {
        return Err(GeneralServerError{message: "user already exists".to_string()})
    }

    let mut transaction = context.db.begin().await?;

    let result = auth_persistence::register_user(&mut transaction, cloned_user_data).await;

    handle_finish_transaction(result, transaction).await
}

/// function returns bool about state of the login attempt
pub async fn login(context: &ExecutionContext, user_data:LoginUserData) -> bool {

    let dbuser = auth_persistence::login_user(context, user_data.clone()).await;

    let user: FilteredUser = match dbuser {
        Ok(u) => u,
        _ => return false
    };

    let is_valid = check_password_hash(user_data.password, user);
    return is_valid
}

/// Function checks if user exists already or not
pub async fn check_user_exists(context: &ExecutionContext, email: String) -> bool {
    let user_exists = auth_persistence::check_user_exists(&context, email).await;
    return match user_exists {
        Ok(res) => {
            res.unwrap_or_else(|| true)
        },
        Err(_) => false
    }
}