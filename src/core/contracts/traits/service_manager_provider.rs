use std::collections::HashMap;
use async_trait::async_trait;
use futures::future::ok;
use serde_json::json;
use sqlx::testing::TestTermination;
use crate::core::client::core::AdministrationClient;
use crate::core::contracts::base::basic_informations::{new_simple_post_body, RequestPostBody, ResponseBody};
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::contracts::dtos::idempotency_info::{IdempotencyEvents, IdempotencyObject};
use crate::core::contracts::traits::services::ClientHandler;
use crate::name_of;

#[async_trait]
pub trait ServiceManagerProvider: Send + Sync {
    async fn handle_command(&self, context: &ExecutionContext, path: &str, request_post_body: RequestPostBody) -> Result<(), GeneralServerError> {
        let admin_client = AdministrationClient {};
        let mut idem_key_obj = IdempotencyObject {
            user_id: Default::default(), // TODO inject user Id sending request into the execution Context in middleware
            idempotency_key: request_post_body.clone().idempotency_key,
            response_status_code: 0,
            response_body: Default::default(),
        };

        let idempotency_handler_body = new_simple_post_body(IdempotencyEvents::CreateIdempotencyKey.name(), json!(idem_key_obj));

        let res = admin_client.handle_command(context, idempotency_handler_body).await;
        return if res.is_ok() {
            let result = self.try_handle_command(context, path, request_post_body).await?;
            if result.is_success() {

                idem_key_obj.response_status_code = 200;
                let idempotency_update_body = new_simple_post_body(
                                                                    IdempotencyEvents::UpdateIdempotencyKey.name(),
                                                                   json!(idem_key_obj));

                admin_client.handle_command(context, idempotency_update_body).await?
            }
            Ok(result)
        } else { res }
    }
    async fn try_handle_command(&self, context: &ExecutionContext, path: &str, request_post_body: RequestPostBody) -> Result<(), GeneralServerError>;
    async fn try_handle_query(&self, context: &ExecutionContext, service: &str, params: HashMap<String, String>) -> Result<ResponseBody, GeneralServerError>;
}