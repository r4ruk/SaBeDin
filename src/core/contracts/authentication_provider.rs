use axum::async_trait;
use crate::core::contracts::dependency_container::ExecutionContext;
use crate::core::contracts::errors::GeneralServerError;
use crate::core::contracts::user::{LoginUserData, RegisterUserData};

#[async_trait]
pub trait AuthProvider: Send + Sync {
    async fn check_user_exists(&self, context: &ExecutionContext, email: String) -> bool;
    async fn register_user(&self, context: &ExecutionContext, create_user_data: RegisterUserData) -> Result<(), GeneralServerError>;
    async fn login(&self, context: &ExecutionContext, login_data: LoginUserData) -> bool;
}