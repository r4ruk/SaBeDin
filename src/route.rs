use std::sync::Arc;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use crate::{DepContainer, request_handler};

pub fn create_router(state: Arc<DepContainer>) -> Router {

    // send everything which ends to a mydomain.com/servicename to the handler function in request_handler
    // servicename then gets handled inside request handler
    Router::new()
        .route("/healthcheck", get(request_handler::request_handler::health_check))
        .route("/*service", post(request_handler::request_handler::command_handler))
        .route("/*service", get(request_handler::request_handler::query_handler))
        .with_state(state)
}