use std::sync::Arc;
use axum::{Router, http::Method, routing::post};
use tower_http::cors::{CorsLayer, Any};
use crate::request_handler;
use crate::core::contracts::base::dependency_container::ExecutionContext;


pub fn auth_routes() -> Router<Arc<ExecutionContext>> {

    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(Any);

    let router = Router::new()
        .route("/register",post(request_handler::auth_handler::create_user_post))
        .route("/login",post(request_handler::auth_handler::login_user_post))
        .route("/logout",post(request_handler::auth_handler::logout_user_post))
        .layer(cors);
    router
}