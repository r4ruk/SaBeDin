mod queue_manager;
mod core;
mod request_handler;
mod service_manager;

use std::sync::Arc;
use axum::{Router, routing::post, routing::get, response::Html};
use crate::service_manager::service_manager::{IServiceManager, ServiceManager, ServiceManagerState};

#[tokio::main]
async fn main() {

    // the ServiceManagerState is used to enable DependencyInjection into the RequestHandler
    let service_manager = ServiceManagerState {
        service_manager: Arc::new(ServiceManager::new())
    };

    // send everything which ends to a mydomain.com/servicename to the handler function in request_handler
    // servicename then gets handled inside request handler
    let app = Router::new()
        .route("/healthcheck", get(request_handler::request_handler::health_check))
        .route("/*service", post(request_handler::request_handler::command_handler))
        .route("/*service", get(request_handler::request_handler::query_handler))
        .with_state(service_manager);

    // listening to address provided for any incoming request.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}