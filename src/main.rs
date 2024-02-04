mod queue_manager;
mod core;
mod request_handler;
mod service_manager;

use std::sync::Arc;
use axum::{Router, routing::post, routing::get, response::Html};
use crate::service_manager::service_manager::{IServiceManager, ServiceManager, ServiceManagerState};

#[tokio::main]
async fn main() {
    let service_manager = ServiceManagerState {
        service_manager: Arc::new(ServiceManager::new())
    };

    let app = Router::new()
        .route("/*service", post(request_handler::request_handler::handler))
        .with_state(service_manager);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
//
//
// async fn handler() -> Html<&'static str> {
//     Html("hello")
// }