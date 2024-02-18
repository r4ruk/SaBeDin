use crate::core::contracts::user::User;
use uuid::Uuid;

pub fn get_user(param: &str) -> User {
    if param.eq("1") {
        return User{
            id: Uuid::new_v4() ,
            name: "hans ueli".to_string(),
            password: "superSafePassword".to_string(),
            email: "hans.ueli@test.ch".to_string(),
            age: 10,
        }
    }
    User{
        id: Default::default(),
        name: "".to_string(),
        password: "".to_string(),
        email: "".to_string(),
        age: 0,
    }
}