use axum::{async_trait, Form, Json, RequestExt};
use axum::extract::{FromRequest, Path, Request, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use crate::core::contracts::basic_informations::{RequestPostBody, ResponseBody};
use crate::service_manager::service_manager::{IServiceManager, ServiceManagerState};


pub async fn health_check() -> Result<String, StatusCode>{
    Ok("healthy".to_string())
}

pub async fn command_handler(State(state): State<ServiceManagerState>,
                     Path(path): Path<String>,
                     JsonOrForm(request_post_body): JsonOrForm<RequestPostBody>) {

    // redirect handling to service manager, which decides what to do with the request.
    state.service_manager.try_handle(path.clone(), request_post_body);
}


pub async fn query_handler(State(state):State<ServiceManagerState>,
                           Path(path): Path<String>) -> Result<Json<ResponseBody>, (StatusCode, Json<ResponseBody>)> {
    Ok(Json::from(ResponseBody { body: format!("hallo von {}", path).to_string() }))
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

    // function used to extract JSON or Form Json out of the request body into the wanted JSON representation
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