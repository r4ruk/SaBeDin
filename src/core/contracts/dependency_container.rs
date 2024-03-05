use std::sync::Arc;
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use crate::config::Config;
use crate::core::client::auth::AuthClient;
use crate::core::contracts::authentication_provider::AuthProvider;
use crate::core::contracts::service_manager_provider::ServiceManagerProvider;
use crate::core::persistence::db_pool;
use crate::service_manager::service_manager::{IServiceManager, ServiceManager};

pub trait DbPoolInitializer {
    async fn init(&self, database_url: &str);
}

pub struct ExecutionContext {
    pub service_manager: Arc<dyn ServiceManagerProvider>,
    pub auth_provider: Arc<dyn AuthProvider>,
    pub env: Config,
    pub db: Pool<Postgres>
    // redis_client: Client,
}

impl ExecutionContext {
    pub async fn new() -> Self {
        dotenv().ok();
        let config = Config::init();

        Self {
            service_manager: Arc::new(ServiceManager::new()),
            auth_provider: Arc::new(AuthClient{}),
            env: config.clone(),
            db: db_pool::init(&config.database_url).await,
        }
    }
}