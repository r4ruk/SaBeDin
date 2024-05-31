use std::collections::HashMap;
use std::string::ToString;
use std::sync::Arc;
use axum::{http::Request, middleware::Next};
use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use axum_extra::extract::CookieJar;
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::ApiError;
use crate::core::contracts::base::token::TokenClaims;
use crate::core::contracts::dtos::user::FilteredUser;
use crate::core::utils::jwt::decode_jwt;

const AUTHORIZATION_HEADER: &str = "Authorization";

pub async fn guard<T>(
    cookie_jar: CookieJar,
    State(_context): State<Arc<ExecutionContext>>,
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

    let result = _context.service_manager.try_handle_query(&*_context, "user", params_map).await;
    if let Ok(body) = result {
        let json_result = serde_json::from_str(&body.body);
        if let Ok(object) = json_result {
            let user: FilteredUser = object;
            // let mut context = _context.lock().await.unwrap();
        }
    }


    // TODO add retrieving user information logic here for permissions and accesscontrol policies
    // to insert identity or user object retrieved via service the following line injects into route:
    // req.extensions_mut().insert(val:identity)
    Ok(next.run(req).await)
}