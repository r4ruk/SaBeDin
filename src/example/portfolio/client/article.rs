use std::collections::HashMap;
use async_trait::async_trait;
use serde_json::{json, Value};
use sqlx::query;
use crate::core::contracts::basic_informations::{QueryOptions, RequestPostBody, ResponseBody};
use crate::core::contracts::dependency_container::ExecutionContext;
use crate::core::contracts::errors::GeneralServerError;
use crate::core::contracts::services::ClientHandler;
use crate::core::persistence::query_builder::Sorting;
use crate::example::portfolio::contracts::article::Article;
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

    /// <inherits />
    ///
    /// single param => returns single element, handle all the same
    /// more params => Returns Vec of elements, handle all the same (also handle queryparams depending
    /// Sorting & paging here.
    async fn handle_query(&self, context: &ExecutionContext, mut params: HashMap<String, String>) -> ResponseBody {
        println!("{:?}", params);

        if params.len() == 1 {
            return match handle_single_param_query(context, &mut params).await {
                Ok(ret) => ResponseBody { body: json!(ret).to_string() },
                Err(e) => ResponseBody { body: json!(e.message).to_string() },
            };
        } else {
            // no param method contributed so it should be defaulted to getall
            if params.contains_key("method") {
                params.insert("method".to_string(), "getall".to_string());
            }
            return match handle_multi_param_query(context, &mut params).await {
                Ok(ret) => ResponseBody { body: json!(ret).to_string() },
                Err(e) => ResponseBody { body: json!(e.message).to_string() },
            };
        }
    }

    fn instantiate() -> Box<dyn ClientHandler> where Self: Sized {
        let client = ArticleClient{};
        return Box::new(client)
    }
}

async fn handle_single_param_query(context: &ExecutionContext, params: &mut HashMap<String, String>) -> Result<Article, GeneralServerError> {
    let result = match params.remove("method") {
        Some(value) => match value.as_str() {
            "programmingkeyname" => crate::example::portfolio::service::article_service::get_article_by_pkn(context, &value.to_string()).await,
            _ => {
                println!("unsupported method");
                Err(GeneralServerError { message: "unsupported method".to_string() })
            }
        },
        None => {
            println!("No method served");
            Err(GeneralServerError { message: "no method given".to_string() })
        }
    };
    result
}

async fn handle_multi_param_query(context: &ExecutionContext, params: &mut HashMap<String, String>) -> Result<Vec<Article>, GeneralServerError> {
    let result = match params.remove("method") {
        Some(value) => match value.as_str() {
            "getall" => crate::example::portfolio::service::article_service::get_all(context, build_query_options_from_params(params.clone())).await,
            _ => {
                println!("unsupported method");
                Err(GeneralServerError { message: "unsupported method".to_string() })
            }
        },
        None => {
            println!("No method served");
            Err(GeneralServerError { message: "no method given".to_string() })
        }
    };
    result
}
