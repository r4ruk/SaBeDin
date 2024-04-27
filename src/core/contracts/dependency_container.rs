use std::sync::Arc;
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use deadpool_lapin::Pool as mq_pool;
use crate::cache::core::basic_cache::Cache;
use crate::config::Config;
use crate::core::client::auth::AuthClient;
use crate::core::contracts::authentication_provider::AuthProvider;
use crate::core::contracts::service_manager_provider::ServiceManagerProvider;
use crate::core::persistence::db_pool;
use crate::queue_manager::manager::QueueManager;
use crate::service_manager::service_manager::{ServiceManager};

pub struct ExecutionContext {
    pub service_manager: Arc<dyn ServiceManagerProvider>,
    pub auth_provider: Arc<dyn AuthProvider>,
    pub env: Config,
    pub db: Pool<Postgres>,
    pub queue: mq_pool,
    pub cache: Cache,
   // redis_client: Client,
}

impl ExecutionContext {
    pub async fn new_with_manager(existing_service_manager: ServiceManager) -> Self {
        dotenv().ok();
        let config = Config::init();
        Self {
            service_manager: Arc::new(existing_service_manager),
            auth_provider: Arc::new(AuthClient{}),
            env: config.clone(),
            db: db_pool::init(&config.database_url).await,
            queue: QueueManager::init().await,
            cache: Cache::initialize()
        }
    }
}