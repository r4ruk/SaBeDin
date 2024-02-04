use axum::{async_trait, Form, Json, RequestExt};
use axum::extract::{FromRequest, Path, Request, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use crate::service_manager::service_manager::{IServiceManager, ServiceManagerState};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    method: String,
    params: String
}

pub async fn handler(State(state): State<ServiceManagerState>,
                     Path(path): Path<String>,
                     JsonOrForm(payload): JsonOrForm<Payload>) {
    state.service_manager.try_handle();
    println!("Path: {:?}", path);
    println!("received payload: {:?}", payload);
}

pub struct JsonOrForm<T>(T);

#[async_trait]
impl<S, T> FromRequest<S> for JsonOrForm<T>
    where
        S: Send + Sync,
        Json<T>: FromRequest<()>,
        Form<T>: FromRequest<()>,
        T: 'static,
{
    type Rejection = Response;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        if let Some(content_type) = content_type {
            if content_type.starts_with("application/json") {
                let Json(payload) = req.extract().await.map_err(IntoResponse::into_response)?;
                return Ok(Self(payload));
            }

            if content_type.starts_with("application/x-www-form-urlencoded") {
                let Form(payload) = req.extract().await.map_err(IntoResponse::into_response)?;
                return Ok(Self(payload));
            }
        }

        Err(StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response())
    }
}