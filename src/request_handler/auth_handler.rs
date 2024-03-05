use std::sync::Arc;
use axum::{
    http::{StatusCode},
    response::IntoResponse,
    Json,
};
use axum::extract::State;
use axum::http::header;
use axum::response::Response;
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde_json::json;
use crate::core::contracts::errors::ApiError;
use crate::core::contracts::user::{RegisterUserData, LoginUserData};
use crate::core::utils::jwt::encode_jwt;
use crate::ExecutionContext;

pub async fn create_user_post(State(context):State<Arc<ExecutionContext>>,
                              Json(user_data): Json<RegisterUserData>
) -> Result<(), ApiError> {

    let existing = context.auth_provider.check_user_exists(context.as_ref(), user_data.email.clone()).await;
    return if !existing {
        let result = context.auth_provider.register_user(context.as_ref(), user_data).await;
        match result {
            Ok(_) => return Ok(()),
            Err(e) => {
                Err(ApiError {
                    message: e.message,
                    redirect: "login".to_string(),
                    status_code: StatusCode::CONFLICT.as_u16()
                })
            }
        }
    } else {
        Err(ApiError {
            message: "User with this email already exists".to_string(),
            redirect: "login".to_string(),
            status_code: StatusCode::CONFLICT.as_u16()
        })
    }
}

pub async fn login_user_post(State(context):State<Arc<ExecutionContext>>,
                             Json(user_data): Json<LoginUserData>)
                             -> Result<impl IntoResponse, ApiError> {
    let is_valid = context.auth_provider.login(context.as_ref(), user_data.clone()).await;
    if !is_valid {
        return Err(ApiError{
            message: "Password or email is wrong".to_string(),
            redirect: "none".to_string(),
            status_code: StatusCode::UNAUTHORIZED.as_u16(),
        })
    } else {
        let encoding_result = encode_jwt(user_data.email.to_string());
        let token: String;
        match encoding_result {
            Ok(t) => token = t,
            Err(_) => return Err(ApiError{
                message: "cant create token for user".to_string(),
                redirect: "none".to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            })
        }
        let cookie = Cookie::build(("token", token.clone()))
            .path("/")
            .max_age(time::Duration::hours(1))
            .same_site(SameSite::Lax)
            .secure(true)
            .http_only(true)
            .build();

        let mut response = Response::new(json!({"status": "success", "token": token}).to_string());
        response
            .headers_mut()
            .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
        Ok(response)
    }
}

pub async fn logout_user_post() -> Result<impl IntoResponse, ApiError> {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .build();

    let mut response = Response::new(json!({"status": "success"}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}