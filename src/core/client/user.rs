use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::{from_str, json};

use crate::core::contracts::base::basic_informations::{RequestPostBody, ResponseBody};
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::contracts::dtos::user::User;
use crate::core::contracts::traits::services::ClientHandler;
use crate::core::service::user_service;
use crate::logger::core_logger::{get_logger, LoggingLevel};

pub struct UserClient {}

#[async_trait]
impl ClientHandler for UserClient {
    async fn handle_command(&self, _context: &ExecutionContext, body: RequestPostBody) -> Result<ResponseBody, GeneralServerError> {
        // TODO add real data and functionality
        println!("handling request in clientservice {:?}", body);

        let user = from_str::<User>(&body.object);
        if user.is_err() == false {
            // ensuring the deserialization worked
            println!("in handlecommand, not handling command yet.");
            return Ok(ResponseBody { body: "handled user post command".to_string() })
        } else {
            let err = GeneralServerError{message:"could not deserialize body".into()};
            let logger = get_logger();
            logger.lock().unwrap().log_message(err.clone(), LoggingLevel::Error);
            Err(err)
        }
    }

    // method creates a boxed instance of the actual Service
    fn instantiate() -> Box<dyn ClientHandler> {
        let c = UserClient { };
        return Box::new(c)
    }

    async fn handle_single_param_query(&self, context: &ExecutionContext, mut params: HashMap<String, String>) -> Result<ResponseBody, GeneralServerError> {
        let result = match params.remove("method") {
            Some(value) => match value.as_str() {
                "email" => {
                    Ok(ResponseBody{
                        body: json!(user_service::get_user_by_email(context, &value.to_string()).await).to_string()
                    })
                },
                _ => {
                    let err = GeneralServerError { message: "unsupported method".to_string() };
                    let logger = get_logger();
                    logger.lock().unwrap().log_message(err.clone(), LoggingLevel::Error);
                    Err(err)
                }
            },
            None => {
                let err = GeneralServerError { message: "no method provided".to_string() };
                let logger = get_logger();
                logger.lock().unwrap().log_message(err.clone(), LoggingLevel::Error);
                Err(err)
            }
        };
        result
    }


    async fn handle_multi_param_query(&self, _context: &ExecutionContext, mut params: HashMap<String, String>) -> Result<ResponseBody, GeneralServerError> {
        let result = match params.remove("method") {
            Some(value) => match value.as_str() {
                _ => {
                    let err = GeneralServerError { message: "unsupported method".to_string() };
                    let logger = get_logger();
                    logger.lock().unwrap().log_message(err.clone(), LoggingLevel::Error);
                    Err(err)
                }
            },
            None => {
                let err = GeneralServerError { message: "no method provided".to_string() };
                let logger = get_logger();
                logger.lock().unwrap().log_message(err.clone(), LoggingLevel::Error);
                Err(err)
            }
        };
        result
    }
}
