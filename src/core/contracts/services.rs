use std::collections::HashMap;
use async_trait::async_trait;
use crate::core::contracts::basic_informations::{RequestPostBody, ResponseBody};
use crate::core::contracts::dependency_container::ExecutionContext;

// definition for a Service, which can be used in the ServiceManager to route requests to.
#[async_trait]
pub trait ClientHandler: 'static + Send + Sync {
    async fn handle_command(&self, context: &ExecutionContext, body: RequestPostBody);
    async fn handle_query(&self, context: &ExecutionContext, params: HashMap<String, String>) -> ResponseBody;
    fn instantiate() -> Box<dyn ClientHandler> where Self: Sized;
}
