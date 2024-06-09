use std::sync::Arc;

use axum::{middleware, Router};

use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::routes::{auth_routes, middlewares};
use crate::routes::main_route::guarded_routes;

pub fn initialize_app(state: Arc<ExecutionContext>, additional_router: Router<Arc<ExecutionContext>>) -> Router {
    // the route layer middleware guard is only applying to the routes which are merged before it.
    // everything after is not guarded by (in this first case) the authentication guard
    let router = Router::new()
        .merge(guarded_routes(state.clone()))
        .route_layer(middleware::from_fn_with_state(state.clone(), middlewares::guard::guard))
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
