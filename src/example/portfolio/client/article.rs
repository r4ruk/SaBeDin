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
    async fn handle_command(&self, context: &ExecutionContext, body: RequestPostBody) {
        // TODO add handle command
        // publish article
        // correct article
        // delete article
    }

    // /// <inherits />
    // ///
    // /// single param => returns single element, handle all the same
    // /// more params => Returns Vec of elements, handle all the same (also handle queryparams depending
    // /// Sorting & paging here.
    // async fn handle_query(&self, context: &ExecutionContext, mut params: HashMap<String, String>) -> ResponseBody {
    //     println!("{:?}", params);
    //
    //     if params.len() == 1 {
    //         return match handle_single_param_query(context, &mut params).await {
    //             Ok(ret) => ResponseBody { body: json!(ret).to_string() },
    //             Err(e) => ResponseBody { body: json!(e.message).to_string() },
    //         };
    //     } else {
    //         // no param method contributed so it should be defaulted to getall
    //         if params.contains_key("method") {
    //             params.insert("method".to_string(), "getall".to_string());
    //         }
    //         return match handle_multi_param_query(context, &mut params).await {
    //             Ok(ret) => ResponseBody { body: json!(ret).to_string() },
    //             Err(e) => ResponseBody { body: json!(e.message).to_string() },
    //         };
    //     }
    // }

    fn instantiate() -> Box<dyn ClientHandler> where Self: Sized {
        let client = ArticleClient{};
        return Box::new(client)
    }
    async fn handle_single_param_query(&self, context: &ExecutionContext, mut params: HashMap<String, String>) -> Result<ResponseBody, GeneralServerError> {
        let result = match params.remove("method") {
            Some(value) => match value.as_str() {
                "programmingkeyname" => {
                    Ok(ResponseBody { body: json!(crate::example::portfolio::service::article_service::get_article_by_pkn(context, &value.to_string()).await).to_string()})
                },
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

    async fn handle_multi_param_query(&self, context: &ExecutionContext, mut params: HashMap<String, String>) -> Result<ResponseBody, GeneralServerError> {
        let result = match params.remove("method") {
            Some(value) => match value.as_str() {
                "getall" => {
                    Ok(ResponseBody{
                        body: json!(article_service::get_all(context, build_query_options_from_params(params.clone())).await).to_string()
                    })
                },
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
}


