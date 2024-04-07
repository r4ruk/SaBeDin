mod queue_manager;
mod core;
mod request_handler;
mod service_manager;
mod config;
mod routes;
mod example;

use std::sync::Arc;
use crate::core::app;
use crate::core::app::start_server;
use crate::core::contracts::dependency_container::ExecutionContext;


#[tokio::main]
async fn main() {


    let service_manager = app::initialize_service_manager().await;
    let state = Arc::new(ExecutionContext::new_with_manager(service_manager).await);

    let app = app::initialize_app(state, Default::default());
    start_server(app).await;


    // let app =
    //     Router::new()
    //         .merge(guarded_routes(state.clone()))
    //         .route_layer(middleware::from_fn_with_state(state.clone(), middlewares::guard::guard::<Body>))
    //         .merge(auth_routes::auth_routes().with_state(state.clone()));

    // listening to address provided for any incoming request.
    // let listener = tokio::net::TcpListener::bind("127.0.0.1:7878")
    //     .await
    //     .unwrap();
    // println!("listening on {}", listener.local_addr().unwrap());
    // axum::serve(listener, app).await.unwrap();
}