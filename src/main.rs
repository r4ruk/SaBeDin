mod queue_manager;
mod core;
mod request_handler;
mod service_manager;
mod config;
mod routes;

use std::sync::Arc;
use axum::{middleware, Router};
use axum::body::Body;
use crate::routes::main_route::guarded_routes;
use crate::routes::{auth_routes, middlewares};
use crate::core::contracts::dependency_container::ExecutionContext;


#[tokio::main]
async fn main() {

    let state = Arc::new(ExecutionContext::new().await);

    // the route layer middleware guard is only applying to the routes which are merged before it.
    // everything after is not guarded by (in this first case) the authentication guard
    let app =
        Router::new()
            .merge(guarded_routes(state.clone()))
            .route_layer(middleware::from_fn_with_state(state.clone(), middlewares::guard::guard::<Body>))
            .merge(auth_routes::auth_routes().with_state(state.clone()));

    // listening to address provided for any incoming request.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}