use std::sync::Arc;

use axum::Router;

use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::example::portfolio::client;
use crate::service_manager::service_manager::SERVICE_MANAGER;

#[allow(unused)]
fn initialize_router(state: Arc<ExecutionContext>) -> Router{
    Router::new().with_state(state)
}

#[allow(unused)]
pub async fn register_clients()  {
    SERVICE_MANAGER
        .register_service("article".to_string(),
                          Box::new(client::article::ArticleClient {})).await;
}