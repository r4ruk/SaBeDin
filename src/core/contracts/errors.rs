use axum::body::Body;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub redirect: String,
    pub status_code: u16,
}

pub struct GeneralServerError {
    pub message: String
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        Response::builder()
            .status(self.status_code)
            .body(Body::from(json!(self).to_string()))
            .unwrap()


    }
}

