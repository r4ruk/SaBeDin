use std::collections::HashMap;
use sqlx::testing::TestTermination;
use crate::core::contracts::base::basic_informations::{RequestPostBody, ResponseBody};
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::contracts::dtos::idempotency_info::IdempotencyEvents;
use crate::core::service;
use crate::logger::core_logger::{get_logger, LoggingLevel};

pub struct IdempotencyClient {}

impl IdempotencyClient {
    pub async fn handle_command(&self, context: &ExecutionContext, body: RequestPostBody) -> Result<(), GeneralServerError>{

        println!("handling request in administrationclient {:?}", body);
        
        match body.method.parse::<IdempotencyEvents>() {
            Ok(event) =>{
                return match event {
                    IdempotencyEvents::CreateIdempotencyKey => {
                        let res = service::administration_service::create_idempotency_key(&context, body.object).await?;
                        if res.is_success() {
                            Ok(())
                        } else {
                            let err = GeneralServerError { message: "Couldnt create idempotency key".to_string() };
                            let logger = get_logger();
                            logger.lock().unwrap().log_error(err.clone(), LoggingLevel::Error);
                            Err(err)
                        }
                    }
                    IdempotencyEvents::UpdateIdempotencyKey => {
                        let res = service::administration_service::update_idempotency_key(&context, body.object).await;
                        if res.is_success() {
                            Ok(())
                        } else {
                            let err = GeneralServerError { message: "Couldnt update idempotency key".to_string() };
                            let logger = get_logger();
                            logger.lock().unwrap().log_error(err.clone(), LoggingLevel::Error);
                            Err(err)
                        }
                    }
                }
            }
            _ => {
                let logger = get_logger();
                let err = GeneralServerError { message: "unrecognized method".to_string() };
                logger.lock().unwrap().log_error(err.clone(), LoggingLevel::Information);
                return Err(err)
            }
        }

    }

    // handles query function for administration functionality
    async fn handle_query(&self, context: &ExecutionContext, params: HashMap<String, String>) -> ResponseBody {
        println!("{:?}", params);
        // TODO handle query if needed...
        return ResponseBody{body: "error".to_string()}
    }
}
