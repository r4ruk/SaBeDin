use std::collections::HashMap;
use async_trait::async_trait;
use crate::core::contracts::base::basic_informations::{RequestPostBody, ResponseBody};
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;

// definition for a Service, which can be used in the ServiceManager to route requests to.
#[async_trait]
pub trait ClientHandler: 'static + Send + Sync {
    async fn handle_query(&self, context: &ExecutionContext, params: HashMap<String, String>) -> ResponseBody {
        let result = self.handle_query_internal(context, params).await;
        if result.is_ok() {
            ResponseBody{
                body: result.unwrap().body,
            }
        } else { 
            ResponseBody {
                body: result.err().unwrap().message,
            }
        }
    }

    fn instantiate() -> Box<dyn ClientHandler> where Self: Sized;
    async fn handle_command(&self, context: &ExecutionContext, body: RequestPostBody) -> Result<ResponseBody, GeneralServerError>;
    async fn handle_query_internal(&self, context: &ExecutionContext, params: HashMap<String, String>) -> Result<ResponseBody, GeneralServerError>;
}