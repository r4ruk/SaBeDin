use std::collections::HashMap;
use async_trait::async_trait;
use serde_json::json;
use crate::core::contracts::basic_informations::{RequestPostBody, ResponseBody};
use crate::core::contracts::dependency_container::ExecutionContext;
use crate::core::contracts::errors::GeneralServerError;
use crate::core::contracts::services::ClientHandler;
use crate::example::portfolio::contracts::article::Article;

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
    async fn handle_query(&self, context: &ExecutionContext, params: HashMap<String, String>) -> ResponseBody {
        println!("{:?}", params);
        if params.len() == 1 {
            let (key, val) = params.iter().nth(0).unwrap();
            let ret_val: Result<Article, GeneralServerError> = match key.as_str() {
                "programmingKeyName" => crate::example::portfolio::service::article_service::get_article_by_pkn(context, val).await,
                _ => {
                    println!("unsupported parameters");
                    Err(GeneralServerError{ message: "unsupported params".to_string() })
                }
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