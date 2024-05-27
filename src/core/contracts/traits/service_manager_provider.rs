use std::collections::HashMap;
use async_trait::async_trait;
use futures::future::ok;
use serde_json::json;
use crate::core::client::core::AdministrationClient;
use crate::core::contracts::base::basic_informations::{RequestPostBody, ResponseBody};
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::contracts::dtos::idempotency_info::IdempotencyObject;
use crate::core::contracts::traits::services::ClientHandler;

#[async_trait]
pub trait ServiceManagerProvider: Send + Sync {
    async fn handle_command(&self, context: &ExecutionContext, path: &str, request_post_body: RequestPostBody) -> Result<(), GeneralServerError> {
        let admin_client = AdministrationClient{};
        let idem_key_obj = IdempotencyObject {
            user_id: Default::default(), // TODO inject user Id sending request into the execution Context in middleware
            idempotency_key: request_post_body.clone().idempotency_key,
            response_status_code: 0,
            response_body: Default::default(),
        };

        let idempotency_handler_body = RequestPostBody {
            idempotency_key: "".to_string(),
            method: "idempotency_key".to_string(),
            object: json!(idem_key_obj).to_string(),
            params: Default::default(),
            query_options: Default::default(),
        };
        let res = admin_client.handle_command(context, idempotency_handler_body).await;
        return if res.is_ok() {
            let result = self.try_handle_command(context, path, request_post_body).await?;
            Ok(result)
        } else { res }
    }
    async fn try_handle_command(&self, context: &ExecutionContext, path: &str, request_post_body: RequestPostBody) -> Result<(), GeneralServerError>;
    async fn try_handle_query(&self, context: &ExecutionContext, service: &str, params: HashMap<String, String>) -> Result<ResponseBody, GeneralServerError>;
}