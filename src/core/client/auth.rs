use async_trait::async_trait;
use crate::core::contracts::authentication_provider::AuthProvider;
use crate::core::contracts::errors::GeneralServerError;
use crate::core::contracts::user::{LoginUserData, RegisterUserData};
use crate::core::service::authentication_service;

pub struct AuthClient{}

#[async_trait]
impl AuthProvider for AuthClient {
    async fn check_user_exists(&self, email: String) -> bool {
        return false
    }
    async fn register_user(&self, create_user_data: RegisterUserData) -> Result<(), GeneralServerError> {
        return authentication_service::register_user(create_user_data).await
    }
    async fn login(&self, login_data: LoginUserData) -> bool {
        let attempt_success = authentication_service::login(login_data).await;
        return attempt_success
    }
}

