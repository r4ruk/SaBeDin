use axum::async_trait;
use crate::core::contracts::errors::GeneralServerError;
use crate::core::contracts::user::{LoginUserData, RegisterUserData};

#[async_trait]
pub trait AuthProvider: Send + Sync {
    async fn check_user_exists(&self, email: String) -> bool;
    async fn register_user(&self, create_user_data: RegisterUserData) -> Result<(), GeneralServerError>;
    async fn login(&self, login_data: LoginUserData) -> bool;
}