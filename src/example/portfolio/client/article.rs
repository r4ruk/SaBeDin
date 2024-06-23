use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::json;

use crate::core::contracts::base::basic_informations::{RequestPostBody, ResponseBody};
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::contracts::traits::services::ClientHandler;
use crate::example::portfolio::service::article_service;
use crate::logger::core_logger::{get_logger, LoggingLevel};
use crate::service_manager::params_object_builder::build_query_options_from_params;

pub struct ArticleClient{}

#[async_trait]
impl ClientHandler for ArticleClient {
    fn instantiate() -> Box<dyn ClientHandler> where Self: Sized {
        let client = ArticleClient{};
        return Box::new(client)
    }

    async fn handle_command(&self, _context: &ExecutionContext, _body: RequestPostBody) -> Result<ResponseBody, GeneralServerError> {
        // TODO add handle command
        // publish article
        // correct article
        // delete article
        return Ok(ResponseBody { body: "all good".to_string() })
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
        if let Some(query) = build_query_options_from_params(params.clone()) {
            // Query params contributed which should be handled by a getall method which is not needed ATM
            return Ok(ResponseBody{
                body: json!(article_service::get_all(context, Some(query)).await?).to_string()})
        }
        // specific query parameters contributed which result in special function implementation
        if let Some(pkn) = params.remove("programmingkeyname") {
            return Ok(ResponseBody{
                body: json!(article_service::get_article_by_pkn(context, &pkn.to_string()).await).to_string()
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


