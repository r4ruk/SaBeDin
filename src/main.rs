mod queue_manager;
mod core;
mod request_handler;
mod service_manager;
mod config;
mod routes;
mod example;
mod cache;
mod logger;
mod tests;

use std::sync::Arc;
use axum::{middleware, Router};
use crate::core::app;
use crate::core::app::start_server;
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::routes::main_route::guarded_routes;
use crate::routes::{auth_routes, middlewares};


#[tokio::main]
async fn main() {

    let state = Arc::new(ExecutionContext::new().await);

    let app = app::initialize_app(state.clone(), Default::default());
    start_server(app).await;


    let app =
        Router::new()
            .merge(guarded_routes(state.clone()))
            .route_layer(middleware::from_fn_with_state(state.clone(), middlewares::guard::guard))
            .merge(auth_routes::auth_routes().with_state(state.clone()));

    // listening to address provided for any incoming request.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}