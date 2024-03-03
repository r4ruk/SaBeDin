use axum::body::Body;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Error;

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


impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        Response::builder()
            .status(self.status_code)
            .body(Body::from(json!(self).to_string()))
            .unwrap()
    }
}

