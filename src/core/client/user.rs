use std::collections::HashMap;
use async_trait::async_trait;
use serde_json::{from_str, json};
use crate::core::contracts::base::basic_informations::{RequestPostBody, ResponseBody};
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::contracts::traits::services::ClientHandler;
use crate::core::contracts::dtos::user::User;
use crate::core::service;
use crate::logger::core_logger::{get_logger, LoggingLevel};

pub struct UserClient {}

#[async_trait]
impl ClientHandler for UserClient {
    async fn handle_command(&self, context: &ExecutionContext, body: RequestPostBody) {
        // TODO add real data and functionality
        println!("handling request in clientservice {:?}", body);

        let user = from_str::<User>(&body.object);
        if user.is_err() == false {
            // ensuring the deserialization worked
            println!("in handlecommand, not handling command yet.");
        } else {
            let logger = get_logger();
            logger.lock().unwrap().log_error(GeneralServerError{message:"could not deserialize body".into()}, LoggingLevel::Error);
        }
    }

    // handles query function for user
    async fn handle_query(&self, context: &ExecutionContext, params: HashMap<String, String>) -> ResponseBody {
        println!("{:?}", params);
        if params.len() == 1 {
            let (key, val) = params.iter().nth(0).unwrap();
            let mut ret_val: Option<User> = None;
            match key.as_str() {
                "id" => ret_val = Some(service::user_service::get_user(val)),
                "email" => ret_val = Some(service::user_service::get_user(val)),
                _ => {
                    let logger = get_logger();
                    logger.lock().unwrap().log_error(GeneralServerError{message:"wrong param given".into()}, LoggingLevel::Warning);
                }
            }
            if let Some(ret) = ret_val {
                return ResponseBody{body: json!(ret).to_string()}
            }
        }
        let logger = get_logger();
        logger.lock().unwrap().log_error(GeneralServerError{message:"no param given".into()}, LoggingLevel::Warning);

        return ResponseBody{body: "error".to_string()}
    }

    // method creates a boxed instance of the actual Service
    fn instantiate() -> Box<dyn ClientHandler> {
        let c = UserClient { };
        return Box::new(c)
    }
}
