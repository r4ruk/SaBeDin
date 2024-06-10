use std::collections::HashMap;
use async_trait::async_trait;
use serde_json::json;
use crate::core::contracts::base::basic_informations::{RequestPostBody, ResponseBody};
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;

// definition for a Service, which can be used in the ServiceManager to route requests to.
#[async_trait]
pub trait ClientHandler: 'static + Send + Sync {
    async fn handle_command(&self, context: &ExecutionContext, body: RequestPostBody);
    async fn handle_query(&self, context: &ExecutionContext, mut params: HashMap<String, String>) -> ResponseBody {
        if params.len() == 1 {
            return self.handle_single_param_query(context, params.clone()).await
                .unwrap_or_else(|e|
                    ResponseBody { body: json!(e.message).to_string() });
        } else {
            // no param method contributed so it should be defaulted to getall
            if params.contains_key("method") {
                params.insert("method".to_string(), "getall".to_string());
            }
            return self.handle_multi_param_query(context, params.clone()).await
                .unwrap_or_else(|e|
                    ResponseBody { body: json!(e.message).to_string() });
        }
    }
    fn instantiate() -> Box<dyn ClientHandler> where Self: Sized;

    async fn handle_single_param_query(&self, context: &ExecutionContext, params: HashMap<String, String>)
        -> Result<ResponseBody, GeneralServerError>;
     async fn handle_multi_param_query(&self, context: &ExecutionContext, params: HashMap<String, String>)
         -> Result<ResponseBody, GeneralServerError>;
}
