use axum::{http::Request, middleware::Next};
use axum::body::Body;
use axum::http::{header, StatusCode};
use axum::response::Response;
use crate::core::contracts::errors::ApiError;
use crate::core::utils::jwt::decode_jwt;

pub async fn guard<T>(mut req: Request<Body>, next:Next) -> Result<Response, ApiError>{
    let token_option:Result<String, ApiError> = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        }).ok_or(ApiError {
        message: "No authorization token provided".to_string(),
        redirect: "none".to_string(),
        status_code: StatusCode::BAD_REQUEST.as_u16()
    });
    let mut token = None;
    match token_option {
        Ok(val) => token = Some(val),
        _ => token = None
    };
    if token == None {
        return Err(ApiError{
            message: "Invalid token provided".to_string(),
            redirect: "none".to_string(),
            status_code: StatusCode::UNAUTHORIZED.as_u16()
        })
    }

    let claim = decode_jwt(token.unwrap()).map_err(|_| ApiError {
        message: "claim not valid".to_string(),
        redirect: "none".to_string(),
        status_code: StatusCode::BAD_REQUEST.as_u16(),
    })?.claims;


    // TODO add retrieving user information logic here for permissions and accesscontrol policies
    // to insert identity or user object retrieved via service the following line injects into route:
    // req.extensions_mut().insert(val:identity)
    Ok(next.run(req).await)
}