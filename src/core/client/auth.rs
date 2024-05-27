use async_trait::async_trait;
use crate::core::contracts::traits::authentication_provider::AuthProvider;
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::contracts::dtos::user::{LoginUserData, RegisterUserData};
use crate::core::service::authentication_service;

pub struct AuthClient{}

#[async_trait]
impl AuthProvider for AuthClient {
    async fn check_user_exists(&self, context: &ExecutionContext, email: String) -> bool {
        return authentication_service::check_user_exists(context, email).await
    }
    async fn register_user(&self, context: &ExecutionContext, create_user_data: RegisterUserData) -> Result<(), GeneralServerError> {
        return authentication_service::register_user(context, create_user_data).await
    }
    async fn login(&self, context: &ExecutionContext, login_data: LoginUserData) -> bool {
        let attempt_success = authentication_service::login(context, login_data).await;
        return attempt_success
    }
}

