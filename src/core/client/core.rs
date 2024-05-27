use std::collections::HashMap;
use crate::core::contracts::base::basic_informations::{RequestPostBody, ResponseBody};
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::service;
use crate::logger::core_logger::{get_logger, LoggingLevel};

pub struct AdministrationClient {}

impl AdministrationClient {
    pub async fn handle_command(&self, context: &ExecutionContext, body: RequestPostBody) -> Result<(), GeneralServerError>{

        println!("handling request in administrationclient {:?}", body);
        
        match body.method.as_str() {
            "createIdempotencyKey" => {
                // TODO deserialize RequestPostBody object which contains the IdempotencyKeyObject
                let res = service::administration_service::create_idempotency_key(body.idempotency_key.clone()).await?;
                return Ok(())
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
