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
use crate::service_manager::params_object_builder::build_query_options_from_params;

pub struct UserClient {}

#[async_trait]
impl ClientHandler for UserClient {
    // method creates a boxed instance of the actual Service
    fn instantiate() -> Box<dyn ClientHandler> {
        let c = UserClient { };
        return Box::new(c)
    }

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

    async fn handle_query_internal(&self, context: &ExecutionContext, mut params: HashMap<String, String>) -> Result<ResponseBody, GeneralServerError> {
        // method contributed in query string
        if let Some(method) = params.remove("method") {
            // if at one point it would be needed to be used, here you go
            let warning = GeneralServerError { message: format!("no valid method implementation found for: {}", method) };
            let logger = get_logger();
            logger.lock().unwrap().log_message(warning.clone(), LoggingLevel::Warning);
            return Err(warning)
        }

        // query params contributed in query
        if let Some(_query) = build_query_options_from_params(params.clone()) {
            // Query params contributed which should be handled by a getall method which is not needed ATM
            let info = GeneralServerError { message: "no valid method implementation found for: get_all".to_string() };
            let logger = get_logger();
            logger.lock().unwrap().log_message(info.clone(), LoggingLevel::Information);
            return Err(info)
        }

        // specific query parameters contributed which result in special function implementation
        if let Some(email) = params.remove("email") {
            return Ok(ResponseBody{
                body: json!(user_service::get_user_by_email(context, &email.to_string()).await).to_string()
            })
        }

        // nothing contributed
        else {
            // TODO shouldnt this default to getall...?

            let err = GeneralServerError {
                message: "No valid method implementation found".to_string(),
            };
            let logger = get_logger();
            logger.lock().unwrap().log_message(err.clone(), LoggingLevel::Error);
            Err(err)
        }

    }
}
