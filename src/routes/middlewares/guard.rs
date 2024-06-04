use std::collections::HashMap;
use std::string::ToString;
use std::sync::Arc;
use axum::{http::Request, middleware::Next};
use axum::body::{Body, to_bytes};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use axum_extra::extract::CookieJar;
use serde_json::Value;
use crate::core::contracts::base::basic_informations::RequestPostBody;
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::{ApiError, GeneralServerError};
use crate::core::contracts::base::token::TokenClaims;
use crate::core::contracts::dtos::user::FilteredUser;
use crate::core::utils::jwt::decode_jwt;

const AUTHORIZATION_HEADER: &str = "Authorization";

pub async fn guard<T>(
    cookie_jar: CookieJar,
    State(context): State<Arc<ExecutionContext>>,
    mut req: Request<Body>,
    next:Next)
    -> Result<Response, ApiError>
{
    let token_option = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(AUTHORIZATION_HEADER)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });


    let token = match token_option {
        Some(val) => Some(val),
        _ => None
    };

    if token == None {
        return Err(ApiError{
            message: "Invalid token provided".to_string(),
            redirect: "none".to_string(),
            status_code: StatusCode::UNAUTHORIZED.as_u16()
        })
    }

    let _claim: TokenClaims = decode_jwt(token.unwrap()).map_err(|_| ApiError {
        message: "claim not valid".to_string(),
        redirect: "none".to_string(),
        status_code: StatusCode::BAD_REQUEST.as_u16(),
    })?.claims;

    let mut params_map = HashMap::new();
    params_map.insert("email".to_string(), _claim.email);

    let new_req = Request::builder();
    let result = context.service_manager.try_handle_query(&*context, "user", params_map).await;
    if let Ok(body) = result {
        let json_result = serde_json::from_str(&body.body);
        if let Ok(object) = json_result {
            let user: FilteredUser = object;
            // let mut context = _context.lock().await.unwrap();

            // Read the request body
            let (parts, body) = req.into_parts();

            let whole_body = to_bytes(body, usize::MAX).await.map_err(|_| ApiError{ message: "INTERNAL SERVER ERRROR".to_string(), redirect: "".to_string(), status_code: 404 })?;

            // Process the body
            // let modified_body = TODO create process_body(whole_body).await.map_err(|_| StatusCode::BAD_REQUEST)?; METHOD
            let mut json: Value = serde_json::from_slice(&whole_body).map_err(|_| ApiError{ message: "BAD REQUEST".to_string(), redirect: "".to_string(), status_code: 404 })?;

            // // Modify the JSON object
            if let Some(obj) = json.as_object_mut() {
                obj.insert("requesting_user_id".to_string(), Value::String(user.id.to_string()));
            }
            let modified_body = serde_json::to_vec(&json).map_err(|_| ApiError{ message: "error".to_string(), redirect: "".to_string(), status_code: 404 })?;

            let new_req = Request::from_parts(parts, Body::from(modified_body));
            Ok(next.clone().run(new_req).await)
        } else {
            Err(ApiError{ message: "error".to_string(), redirect: "".to_string(), status_code: 404 })
        }
    } else {
        // TODO not successfully found user, so he is unauthorized to access
        Err(ApiError{ message: "error".to_string(), redirect: "".to_string(), status_code: 404 })
        // Ok(next.run(req).await)
    }
}