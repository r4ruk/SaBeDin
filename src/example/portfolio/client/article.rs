use std::collections::HashMap;
use async_trait::async_trait;
use serde_json::json;
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
    /// TODO handle multiparams in a nicer way -> Get all can/should have multiple params definitely,
    /// TODO should also be able to give query, sorting, paging in params. handle this....
    ///
    /// single param => returns single element, handle all the same
    /// more params => Returns Vec of elements, handle all the same (also handle queryparams depending
    /// Sorting & paging here.
    /// Split again programmingkeyName and getall match as they return different results.
    ///
    async fn handle_query(&self, context: &ExecutionContext, params: HashMap<String, String>) -> ResponseBody {
        println!("{:?}", params);
        if params.len() == 1 {
            let (key, val) = params.iter().nth(0).unwrap();
            let ret_val: Result<Article, GeneralServerError> = match key.as_str() {
                "programmingkeyname" => {
                    crate::example::portfolio::service::article_service::get_article_by_pkn(context, val).await
                },
                _ => {
                    println!("unsupported parameters");
                    Err(GeneralServerError{ message: "unsupported params".to_string() })
                }
            };
            if let Ok(ret) = ret_val {
                return ResponseBody{body: json!(ret).to_string()}
            }
        } else { // multiple params so expect to have multiple results.
            let (firstkey, firstval) = params.iter().nth(0).unwrap();
            let ret_val = match firstkey.as_str() {
                "getall" => crate::example::portfolio::service::article_service::get_all(context, build_query_options_from_params(params.clone())).await,
                _ => Err(GeneralServerError{ message: "unsupported function or params".to_string() })
            };
            if let Ok(ret) = ret_val {
                return ResponseBody{body: json!(ret).to_string()}
            }
        }
        return ResponseBody{body: "error".to_string()}
    }

    fn instantiate() -> Box<dyn ClientHandler> where Self: Sized {
        let client = ArticleClient{};
        return Box::new(client)
    }
}
