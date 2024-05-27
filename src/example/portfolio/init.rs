use std::sync::Arc;
use axum::Router;
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::example::portfolio::client;
use crate::service_manager::service_manager::{ServiceManagerConstruction, ServiceManager};

#[allow(unused)]
fn initialize_router(state: Arc<ExecutionContext>) -> Router{
    Router::new().with_state(state)
}

pub async fn register_clients(mut service_manager: ServiceManager) -> ServiceManager {
    service_manager
        .register_service("article".to_string(),
                          Box::new(client::article::ArticleClient {}))
        .await;
    return service_manager
}