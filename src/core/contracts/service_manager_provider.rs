use std::collections::HashMap;
use async_trait::async_trait;
use crate::core::contracts::basic_informations::{RequestPostBody, ResponseBody};
use crate::core::contracts::dependency_container::ExecutionContext;
use crate::core::contracts::errors::GeneralServerError;

#[async_trait]
pub trait ServiceManagerProvider: Send + Sync {
    async fn try_handle(&self, context: &ExecutionContext,  path: &str, request_post_body: RequestPostBody) -> Result<(), GeneralServerError>;
    async fn try_handle_query(&self, context: &ExecutionContext, service: &str, params: HashMap<String, String>) -> Result<ResponseBody, GeneralServerError>;
}