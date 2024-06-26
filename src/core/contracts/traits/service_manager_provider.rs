use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::json;
use uuid::Uuid;

use crate::core::client::core::IdempotencyClient;
use crate::core::contracts::base::basic_informations::{new_simple_post_body, CommandResponse, RequestPostBody, ResponseBody};
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::contracts::dtos::idempotency_info::{IdempotencyObject, IdempotencyEvents};

#[async_trait]
pub trait ExtendableServiceManagerProvider : ServiceManagerProvider + ExtendableServiceManager + Send + Sync + Sized + 'static{ }

#[async_trait]
pub trait ServiceManagerProvider: Send + Sync + Sized + 'static{
    async fn handle_command(&self, context: &ExecutionContext, path: &str, request_post_body: RequestPostBody, user_id: Uuid) -> Result<CommandResponse, GeneralServerError> {
        let admin_client = IdempotencyClient {};
        let mut idem_key_obj = IdempotencyObject {
            user_id,
            idempotency_key: request_post_body.clone().idempotency_key,
            response_status_code: 0,
            response_body: Default::default(),
        };

        let idempotency_handler_body = new_simple_post_body(IdempotencyEvents::CreateIdempotencyKey.name(), json!(idem_key_obj));

        let res = admin_client.handle_command(context, idempotency_handler_body).await;
        return if res.is_ok() {
            let result = self.try_handle_command(context, path, request_post_body).await?;
            idem_key_obj.response_status_code = result.code;
            idem_key_obj.response_body = json!(result.response);
            let idempotency_update_body = new_simple_post_body(
                IdempotencyEvents::UpdateIdempotencyKey.name(),
                json!(idem_key_obj));

            admin_client.handle_command(context, idempotency_update_body).await?;
            Ok(CommandResponse { code: 200, response: "success".to_string() })
        } else { Err(res.err().unwrap()) }
    }
    async fn try_handle_command(&self, context: &ExecutionContext, path: &str, request_post_body: RequestPostBody) -> Result<CommandResponse, GeneralServerError>;
    async fn try_handle_query(&self, context: &ExecutionContext, service: &str, params: HashMap<String, String>) -> Result<ResponseBody, GeneralServerError>;
}

#[async_trait]
pub trait ExtendableServiceManager : Send + Sync {
    async fn register_external_service(&mut self, service_name: String);
}