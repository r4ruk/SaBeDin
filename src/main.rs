mod queue_manager;
mod core;
mod request_handler;
mod service_manager;
mod route;

use std::sync::Arc;
use axum::{Router, routing::post, routing::get};
use crate::route::create_router;
use crate::service_manager::service_manager::{IServiceManager, ServiceManager, ServiceManagerState};
// use redis::Client;

pub struct DepContainer {
    service_manager: ServiceManager,
    // env: Config,
    // redis_client: Client,
}

#[tokio::main]
async fn main() {

    // the ServiceManagerState is used to enable DependencyInjection into the RequestHandler
    let state = Arc::new(DepContainer {
        service_manager: ServiceManager::new()
    });

    let app = create_router(state);

    // listening to address provided for any incoming request.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}