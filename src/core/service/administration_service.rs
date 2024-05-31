use serde_json::from_str;
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::contracts::dtos::idempotency_info::IdempotencyObject;
use crate::core::persistence::repositories::administration_repository;
use crate::logger::core_logger::{get_logger, LoggingLevel};

/// Creates the idempotency key if it does not exist
pub async fn create_idempotency_key(context: &ExecutionContext, idempotency_json: String) -> Result<(), GeneralServerError> {
    let idem_key_object = from_str::<IdempotencyObject>(&idempotency_json);

    if idem_key_object.is_ok() {
        let idempotency_key_object = idem_key_object.unwrap();
        let res = administration_repository::idempotency_key_exists(&context, idempotency_key_object.clone().idempotency_key, idempotency_key_object.clone().user_id)
            .await?;
        if res.is_some() && res.unwrap() != true {
            let mut transaction = context.db.get_pool().begin().await?;
            administration_repository::create_idempotency_key(&mut transaction, idempotency_key_object).await?;
        } else {
            let err = GeneralServerError {message: "Command already handled".to_string()};
            let logger = get_logger();
            logger.lock().unwrap().log_error(err.clone(), LoggingLevel::Warning);
            return Err(err)
        }
    }
    return Ok(())
}

/// Updates the idempotency key
pub async fn update_idempotency_key(context: &ExecutionContext, idempotency_json: String) -> Result<(), GeneralServerError> {
    let idem_key_object = from_str::<IdempotencyObject>(&idempotency_json);

    if idem_key_object.is_ok() {
        let idempotency_key_object = idem_key_object.unwrap();
        let mut transaction = context.db.get_pool().begin().await?;
        administration_repository::update_idempotency_key(&mut transaction, idempotency_key_object).await?;
    } else {
        let err = GeneralServerError {
            message: "Could not map into IdempotencyObject.".to_string(),
        };
        return Err(err)
    }
    return Ok(())
}