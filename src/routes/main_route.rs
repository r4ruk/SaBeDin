use std::sync::Arc;
use axum::{
    routing::{get, post},
    Router,
};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::Method;
use tower_http::cors::{CorsLayer, Any};
use crate::{ExecutionContext, request_handler};

pub fn guarded_routes(state: Arc<ExecutionContext>) -> Router {

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // send everything which ends to a mydomain.com/servicename to the handler function in request_handler
    // servicename then gets handled inside request handler
    Router::new()
        .route("/healthcheck", get(request_handler::request_handler::health_check))
        .route("/*service", post(request_handler::request_handler::command_handler))
        .route("/*service", get(request_handler::request_handler::query_handler))
        .with_state(state)
        .layer(cors)
}