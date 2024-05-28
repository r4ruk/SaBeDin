use serde_json::from_str;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::contracts::dtos::idempotency_info::IdempotencyObject;

pub async fn create_idempotency_key(idempotency_json: String) -> Result<(), GeneralServerError>{
    let idem_key_object = from_str::<IdempotencyObject>(&idempotency_json);

    if idem_key_object.is_ok() {
        let _obj = idem_key_object.unwrap();
        // todo handle update in persistence
    }
    return Ok(())

}
pub async fn update_idempotency_key(idempotency_json: String) {
    let idem_key_object = from_str::<IdempotencyObject>(&idempotency_json);

    if idem_key_object.is_ok() {
        let _obj = idem_key_object.unwrap();
        // todo handle update in persistence
    }
}