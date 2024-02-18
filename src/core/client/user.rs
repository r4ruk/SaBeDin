use std::collections::HashMap;
use serde_json::{from_str, json};
use crate::core::contracts::basic_informations::{RequestPostBody, ResponseBody};
use crate::core::contracts::services::ClientHandler;
use crate::core::contracts::user::User;
use crate::core::service;

pub struct UserClient {}

impl ClientHandler for UserClient {
    fn handle_command(&self, body: RequestPostBody) {
        // TODO add real data and functionality
        println!("handling request in clientservice {:?}", body);

        // parsing body's object which can be any 'serializable/deserializable' JSON representation.
        let user: User = from_str(&body.object).expect("cant parse body object");

        // ensuring the deserialization worked
        println!("name is {} and age is {}", user.name, user.age)
    }

    // handles query function for user
    fn handle_query(&self, params: HashMap<String, String>) -> ResponseBody {
        println!("{:?}", params);
        if params.len() == 1 {
            let (key, val) = params.iter().nth(0).unwrap();
            let mut ret_val: Option<User> = None;
            match key.as_str() {
                "id" => ret_val = Some(service::user_service::get_user(val)),
                "email" => ret_val = Some(service::user_service::get_user(val)),
                _ => {
                    println!("wrong param given");
                }
            }
            if let Some(ret) = ret_val {
                return ResponseBody{
                    body: json!(ret).to_string()
                }
            }
        }
        return ResponseBody {
            body: "hello from handle_query".to_string(),
        }
    }

    // method creates a boxed instance of the actual Service
    fn instantiate() -> Box<dyn ClientHandler> {
        let c = UserClient { };
        return Box::new(c);
    }
}
