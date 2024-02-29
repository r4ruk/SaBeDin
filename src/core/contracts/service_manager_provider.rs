use std::collections::HashMap;
use crate::core::contracts::basic_informations::{RequestPostBody, ResponseBody};
use crate::core::contracts::dependency_container::ExecutionContext;

pub trait ServiceManagerProvider: Send + Sync  {
    fn try_handle(&self, context: &ExecutionContext,  path: String, request_post_body: RequestPostBody);
    fn try_handle_query(&self, context: &ExecutionContext, service: String, params: HashMap<String, String>) -> ResponseBody;
}