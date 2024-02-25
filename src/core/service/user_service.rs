use chrono::Utc;
use crate::core::contracts::user::User;
use uuid::Uuid;

pub fn get_user(param: &str) -> User {
    if param.eq("1") {
        return User{
            id: Uuid::new_v4(),
            name: "hans ueli".to_string(),
            email: "hans.ueli@test.ch".to_string(),
            password: "superSafePassword".to_string(),
            role: "".to_string(),
            verified: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    return User {
        id: Default::default(),
        name: "".to_string(),
        email: "".to_string(),
        password: "".to_string(),
        role: "".to_string(),
        verified: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}