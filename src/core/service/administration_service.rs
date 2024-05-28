use serde_json::from_str;
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::contracts::dtos::idempotency_info::IdempotencyObject;
use crate::core::persistence::repositories::administration_persistence;
use crate::logger::core_logger::{get_logger, LoggingLevel};

pub async fn create_idempotency_key(context: &ExecutionContext, idempotency_json: String) -> Result<(), GeneralServerError>{
    let idem_key_object = from_str::<IdempotencyObject>(&idempotency_json);

    if idem_key_object.is_ok() {
        let idempotency_key_object = idem_key_object.unwrap();
        let res = administration_persistence::idempotency_key_exists(&context, idempotency_key_object.clone().idempotency_key, idempotency_key_object.clone().user_id)
            .await?;
        if res.is_some() && res.unwrap() != true {
            let mut transaction = context.db.get_pool().begin().await?;
            let result = administration_persistence::create_idempotency_key(&mut transaction, idempotency_key_object).await?;
            // todo check result or throw general server error
        } else {
            let err = GeneralServerError {message: "Command already handled".to_string()};
            let logger = get_logger();
            logger.lock().unwrap().log_error(err.clone(), LoggingLevel::Warning);
            return Err(err)
        }
    }
    return Ok(())

}
pub async fn update_idempotency_key(context: &ExecutionContext, idempotency_json: String) {
    let idem_key_object = from_str::<IdempotencyObject>(&idempotency_json);

    if idem_key_object.is_ok() {
        let _obj = idem_key_object.unwrap();
        // todo handle update in persistence
    }
}