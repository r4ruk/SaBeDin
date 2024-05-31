use std::collections::HashMap;
use async_trait::async_trait;
use serde_json::{from_str, json};
use crate::core::contracts::base::basic_informations::{RequestPostBody, ResponseBody};
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::contracts::traits::services::ClientHandler;
use crate::core::contracts::dtos::user::{FilteredUser, User};
use crate::core::service;
use crate::core::service::user_service;
use crate::example::portfolio::contracts::article::Article;
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
    async fn handle_query(&self, context: &ExecutionContext, mut params: HashMap<String, String>) -> ResponseBody {
        println!("{:?}", params);
        if params.len() == 1 {
            return match handle_single_param_query(context, &mut params).await {
                Ok(ret) => ResponseBody { body: json!(ret).to_string() },
                Err(e) => ResponseBody { body: json!(e.message).to_string() },
            };
        } else {
            // // no param method contributed so it should be defaulted to getall
            // if params.contains_key("method") {
            //     params.insert("method".to_string(), "getall".to_string());
            // }
            // return match handle_multi_param_query(context, &mut params).await {
            //     Ok(ret) => ResponseBody { body: json!(ret).to_string() },
            //     Err(e) => ResponseBody { body: json!(e.message).to_string() },
            // };
            return ResponseBody {body: "not implemented yet".to_string()}
        }
    }

    // method creates a boxed instance of the actual Service
    fn instantiate() -> Box<dyn ClientHandler> {
        let c = UserClient { };
        return Box::new(c)
    }
}



async fn handle_single_param_query(context: &ExecutionContext, params: &mut HashMap<String, String>) -> Result<FilteredUser, GeneralServerError> {
    let result = match params.remove("method") {
        Some(value) => match value.as_str() {
            "email" => user_service::get_user_by_email(context, &value.to_string()).await,
            _ => {
                let err = GeneralServerError { message: "unsupported method".to_string() };
                let logger = get_logger();
                logger.lock().unwrap().log_error(err.clone(), LoggingLevel::Error);
                Err(err)
            }
        },
        None => {
            let err = GeneralServerError { message: "no method provided".to_string() };
            let logger = get_logger();
            logger.lock().unwrap().log_error(err.clone(), LoggingLevel::Error);
            Err(err)
        }
    };
    result
}