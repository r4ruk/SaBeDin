use axum::body::Body;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::broadcast::error::SendError;
use crate::core::contracts::base::system_messages::SysMessage;

#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub redirect: String,
    pub status_code: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorCode {
    Unauthorized,
    BadRequest,
    Forbidden,
    NotFound
}

impl ErrorCode{
    pub fn get_error_code(&self) -> u16{
        match self {
            ErrorCode::Unauthorized => 401,
            ErrorCode::BadRequest => 400,
            ErrorCode::Forbidden => 403,
            ErrorCode::NotFound => 404,
        }
    }
}

impl ApiError {
    pub fn new(err_code: ErrorCode) -> Self{
        return Self {
            message: format!("{:?}", err_code),
            redirect: "".to_string(),
            status_code: err_code.get_error_code(),
        }
    }
}


#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct GeneralServerError {
    pub message: String
}

impl SysMessage for GeneralServerError {
    fn get_internal_message(&self) -> &str {
        return &self.message
    }
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
impl From<GeneralServerError> for ApiError {
    fn from(error: GeneralServerError) -> Self {
        let message = serde_json::json!({
            "status":"fail",
            "message":format!("Error occurred while retrieving informations {:?}", error)
        });
        return ApiError{
            message: message.to_string(),
            redirect: "".to_string(),
            status_code: 404,
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

