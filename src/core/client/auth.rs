use crate::core::contracts::user::{RegisterUserData, LoginUserData};

pub trait AuthProvider: Send + Sync {
    fn check_user_exists(&self, email: String) -> bool;
    fn register_user(&self, create_user_data: RegisterUserData);
    fn login(&self, login_data: LoginUserData) -> bool;
}

pub struct AuthClient{}

impl AuthProvider for AuthClient {
    fn check_user_exists(&self, email: String) -> bool {
        return false
    }
    fn register_user(&self, create_user_data: RegisterUserData) {
        // TODO register user function call to service and so on.
    }
    fn login(&self, login_data: LoginUserData) -> bool {
        // TODO call service login func

        return true
    }
}

