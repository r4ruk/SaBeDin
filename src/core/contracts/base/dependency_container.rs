use std::sync::Arc;
use dotenv::dotenv;
use deadpool_lapin::Pool as mq_pool;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::config::Config;
use crate::core::client::auth::AuthClient;
use crate::core::contracts::traits::authentication_provider::AuthProvider;
use crate::core::contracts::traits::service_manager_provider::ServiceManagerProvider;
use crate::core::persistence::core::db_pool::{DbConnectionPoolProvider, PostgresConnection};
use crate::queue_manager::manager::QueueManager;
use crate::service_manager::service_manager::ServiceManager;

pub struct ExecutionContext {
    pub service_manager: Arc<dyn ServiceManagerProvider>,
    pub auth_provider: Arc<dyn AuthProvider>,
    pub db: Arc<dyn DbConnectionPoolProvider<PoolType=Pool<Postgres>>>,
    pub env: Config,
    pub queue: mq_pool,
    pub requesting_user_id: Uuid
}

impl ExecutionContext {
    pub async fn new_with_manager(existing_service_manager: ServiceManager) -> Self {
        dotenv().ok();
        let config = Config::init();
        Self {
            service_manager: Arc::new(existing_service_manager),
            auth_provider: Arc::new(AuthClient{}),
            db: Arc::new(PostgresConnection::init(&config.database_url).await),
            env: config.clone(),
            queue: QueueManager::init().await,
            requesting_user_id: Default::default()
        }
    }
}