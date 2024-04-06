use std::sync::Arc;
use axum::{middleware, Router};
use axum::body::Body;
use crate::core::contracts::dependency_container::ExecutionContext;
use crate::example;
use crate::routes::{auth_routes, middlewares};
use crate::routes::main_route::guarded_routes;
use crate::service_manager::service_manager::{IServiceManager, ServiceManager};

pub fn initialize_app(state: Arc<ExecutionContext>, additional_router: Router<Arc<ExecutionContext>>) -> Router {
    // the route layer middleware guard is only applying to the routes which are merged before it.
    // everything after is not guarded by (in this first case) the authentication guard
    let mut router = Router::new()
        .merge(guarded_routes(state.clone()))
        .route_layer(middleware::from_fn_with_state(state.clone(), middlewares::guard::guard::<Body>))
        .merge(auth_routes::auth_routes().with_state(state.clone()))
        .merge(additional_router.with_state(state.clone()));
    return router
}

pub async fn start_server(app: Router) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

/// function initialized service manager with project specific clients.
pub async fn initialize_service_manager() -> ServiceManager {
    let mut manager = ServiceManager::new().await;

    // Project specific managers added here
    // init functionality should take a manager, alter it with the new routes and returns it.
    manager = example::portfolio::init::register_clients(manager).await;

    //

    return manager
}