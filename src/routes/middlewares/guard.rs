use std::collections::HashMap;
use std::string::ToString;
use std::sync::Arc;

use axum::{http::Request, middleware::Next};
use axum::body::{Body, Bytes, to_bytes};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use axum_extra::extract::CookieJar;
use serde_json::Value;

use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::{ApiError, ErrorCode};
use crate::core::contracts::base::token::TokenClaims;
use crate::core::contracts::dtos::user::FilteredUser;
use crate::core::contracts::traits::service_manager_provider::ServiceManagerProvider;
use crate::core::utils::jwt::decode_jwt;
use crate::service_manager::service_manager::SERVICE_MANAGER;

const AUTHORIZATION_HEADER: &str = "Authorization";
const IDEMPOTENCYKEY_HEADER: &str = "idempotencykey";

pub async fn guard(
    cookie_jar: CookieJar,
    State(context): State<Arc<ExecutionContext>>,
    req: Request<Body>,
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

    let idempotency_key = req.headers()
        .get(IDEMPOTENCYKEY_HEADER)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            Some(auth_value.to_string())
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

    // try retrieving user
    let result = SERVICE_MANAGER.try_handle_query(&*context, "user", params_map).await?;

    let json_result = serde_json::from_str(&result.body);
    if let Ok(object) = json_result {
        let user: FilteredUser = object;

        // Read the request body
        let (parts, body) = req.into_parts();
        let whole_body = to_bytes(body, usize::MAX).await
            .map_err(|_| ApiError::new(ErrorCode::NotFound))?;

        // Process the body
        let mut modified_body = process_body(user.id.to_string(),
                                         "requesting_user_id".to_string(),
                                         &whole_body).await?;

        let mut new_req = Request::from_parts(parts.clone(), Body::from(modified_body));

        // insert idempotency key
        if let Some(key) = idempotency_key {
            modified_body = process_body(key,
                         "idempotency_key".to_string(),
                         &whole_body).await?;
            new_req = Request::from_parts(parts, Body::from(modified_body));
            Ok(next.clone().run(new_req).await)
        } else {
            Ok(next.clone().run(new_req).await)
        }
    } else {
        Err(ApiError::new(ErrorCode::NotFound))
    }
}

async fn process_body(user_id: String, property_name: String, whole_body: &Bytes) -> Result<Vec<u8>, ApiError> {
    let mut json: Value = if whole_body.is_empty() {
        Value::Object(serde_json::Map::new())
    } else {
        serde_json::from_slice(&whole_body).map_err(|_| ApiError::new(ErrorCode::BadRequest))?
    };

    // // Modify the JSON object
    if let Some(obj) = json.as_object_mut() {
        obj.insert(property_name, Value::String(user_id));
    }
    let modified_body = serde_json::to_vec(&json).map_err(|_| ApiError::new(ErrorCode::BadRequest))?;
    Ok(modified_body)
}