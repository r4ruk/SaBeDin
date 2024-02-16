use std::collections::HashMap;
use axum::{async_trait, Form, Json, RequestExt};
use axum::extract::{FromRequest, Path, Request, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use crate::core::contracts::{basic_informations::{RequestPostBody, ResponseBody}, uri_helper};
use crate::service_manager::service_manager::ServiceManagerState;


pub async fn health_check() -> Result<String, StatusCode>{
    Ok("healthy".to_string())
}

// handler for POST requests
pub async fn command_handler(State(state): State<ServiceManagerState>,
                     Path(path): Path<String>,
                     JsonOrForm(request_post_body): JsonOrForm<RequestPostBody>) {

    // redirect handling to service manager, which decides what to do with the request.
    state.service_manager.try_handle(path.clone(), request_post_body);
}

// handler for GET-Requests
pub async fn query_handler(State(state):State<ServiceManagerState>,
                           mut req: Request) -> Result<Json<ResponseBody>, (StatusCode, Json<ResponseBody>)> {

    let mut response_body = ResponseBody{ body: "".to_string() };
    println!("{:?}",req.uri());
    let uri_path = req.uri_mut().to_string();
    if uri_path.contains('?') {
        let splitted = uri_path.split_once('?').unwrap();
        let servicename:&str = splitted.0;
        let params = splitted.1;
        let mut service = "".to_string();
        if servicename.contains('/') {
            service = servicename.replace('/', "");
        }

        let params = uri_helper::handle_params(params);

        response_body = state.service_manager.try_handle_query(service.to_string(), params);
    } else if uri_path.contains('/') {
        // TODO generally handle errors
        let error_response = Json::from(ResponseBody{ body: "not found".to_string() });
        let status_code = StatusCode::INTERNAL_SERVER_ERROR;

        let error_result: Result<Json<ResponseBody>, (StatusCode, Json<ResponseBody>)> = Err((status_code, error_response));
        return error_result
    }

    Ok(Json::from(response_body))
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