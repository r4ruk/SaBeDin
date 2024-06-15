use std::str::FromStr;
use std::sync::Arc;
use serde_json::json;
use axum::{async_trait, Form, Json, RequestExt, response::{IntoResponse, Response}, http::{header::CONTENT_TYPE, StatusCode}, extract::{FromRequest, Path, Request, State}, debug_handler};
use uuid::Uuid;
use crate::ExecutionContext;
use crate::core::contracts::{base::basic_informations::{RequestPostBody, ResponseBody}};
use crate::core::contracts::base::basic_informations::RequestPostBodyWrapper;
use crate::core::contracts::base::system_messages::InformationMessage;
use crate::core::contracts::traits::service_manager_provider::ServiceManagerProvider;
use crate::core::utils::uri_helper;
use crate::logger::core_logger::{get_logger, LoggingLevel};
use crate::service_manager::service_manager::SERVICE_MANAGER;


pub async fn health_check() -> Result<String, StatusCode>{
    println!("in healthcheck");
    Ok(json!(ResponseBody{body:"healthy!".to_string()}).to_string())
}
// handler for POST requests
#[debug_handler]
pub async fn register_service(State(context): State<Arc<ExecutionContext>>,
                             Path(path): Path<String>,
                             JsonOrForm(wrapper): JsonOrForm<RequestPostBodyWrapper>) {
    // TODO to be defined how the service registering post body has to look like
    let service = wrapper.object;

    let logger = get_logger();
    logger.lock().unwrap().log_message(InformationMessage{ message: format!("registered microservice in external services: '{}'", service) }, LoggingLevel::Information);

    SERVICE_MANAGER.external_services.write().await.push(service);
}


// handler for POST requests
#[debug_handler]
pub async fn command_handler(State(context): State<Arc<ExecutionContext>>,
                             Path(path): Path<String>,
                             JsonOrForm(wrapper): JsonOrForm<RequestPostBodyWrapper>) {
    // map into RequestPostBody
    let request_post_body = wrapper.clone().into();

    let user_id_result = Uuid::from_str(&wrapper.requesting_user_id);
    if user_id_result.is_err() {
        return;
    }

    // redirect handling to service manager, which decides what to do with the request.
    let result = SERVICE_MANAGER.handle_command(context.as_ref(), &path, request_post_body, user_id_result.unwrap()).await;
    match result {
        Ok(_) => {println!("successfull handled post request")}
        Err(e) => {
            let logger = get_logger();
            logger.lock().unwrap().log_message(e, LoggingLevel::Error);
        }
    }
}

// handler for GET-Requests
#[debug_handler]
pub async fn query_handler(State(context):State<Arc<ExecutionContext>>,
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

        let result = SERVICE_MANAGER.try_handle_query(context.as_ref(), &service, params).await;
        if result.is_ok() {
            response_body.body = result.unwrap().body
        } else {
            response_body.body = result.err().unwrap().message
        }
    } else if uri_path.contains('/') {
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