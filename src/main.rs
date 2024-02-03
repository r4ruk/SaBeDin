mod queue_manager;
mod core;
mod request_handler;
mod service_manager;

use axum::{Router, routing::get, response::Html};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}


async fn handler() -> Html<&'static str> {
    Html("hello")
}