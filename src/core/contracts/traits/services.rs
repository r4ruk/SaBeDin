use std::collections::HashMap;
use async_trait::async_trait;
use crate::core::contracts::base::basic_informations::{RequestPostBody, ResponseBody};
use crate::core::contracts::base::dependency_container::ExecutionContext;

// definition for a Service, which can be used in the ServiceManager to route requests to.
#[async_trait]
pub trait ClientHandler: 'static + Send + Sync {
    // TODO add general basic implementation for handle_query which calls 2 new functions which have to be implemented by each client
    async fn handle_command(&self, context: &ExecutionContext, body: RequestPostBody);
    async fn handle_query(&self, context: &ExecutionContext, params: HashMap<String, String>) -> ResponseBody;
    fn instantiate() -> Box<dyn ClientHandler> where Self: Sized;

    // async fn handle_single_param_query()
    // async fn handle_multi_param_query()
}
