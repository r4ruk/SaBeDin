use std::sync::Arc;
use sqlx::{Pool, Postgres};
use crate::config::Config;
use crate::core::contracts::authentication_provider::AuthProvider;
use crate::core::contracts::service_manager_provider::ServiceManagerProvider;



pub struct ExecutionContext {
    pub service_manager: Arc<dyn ServiceManagerProvider>,
    pub auth_provider: Arc<dyn AuthProvider>,
    pub env: Config,
    pub db: Pool<Postgres>
    // redis_client: Client,
}