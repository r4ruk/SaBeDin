use axum::body::Body;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::broadcast::error::SendError;

#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub redirect: String,
    pub status_code: u16,
}

#[derive(Debug)]
pub struct GeneralServerError {
    pub message: String
}

impl From<sqlx::Error> for GeneralServerError {
    fn from(error: sqlx::Error) -> Self {
        let message = serde_json::json!({
            "status":"fail",
            "message":format!("Error occurred while retrieving informations {:?}", error)
        });
        return GeneralServerError{
            message: message.to_string(),
        }
    }
}

impl From<lapin::Error> for GeneralServerError {
    fn from(error: lapin::Error) -> Self {
        let message = json!({
            "status":"fail",
            "message":format!("Error happened in message queue manager: {:?}", error)
        });
        return GeneralServerError {
            message: message.to_string(),
        }
    }
}
impl From<SendError<String>> for GeneralServerError {
    fn from(error: SendError<String>) -> Self {
        let message = json!({
            "status":"fail",
            "message":format!("Error could not push message to clients. '{}'", error)
        });
        return GeneralServerError {
            message: message.to_string(),
        }
    }
}


impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        Response::builder()
            .status(self.status_code)
            .body(Body::from(json!(self).to_string()))
            .unwrap()
    }
}

