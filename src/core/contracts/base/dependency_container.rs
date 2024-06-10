use std::sync::Arc;

use deadpool_lapin::Pool as mq_pool;
use dotenv::dotenv;
use sqlx::{Pool, Postgres};

use crate::config::Config;
use crate::core::client::auth::AuthClient;
use crate::core::contracts::traits::authentication_provider::AuthProvider;
use crate::core::persistence::core::db_pool::{DbConnectionPoolProvider, PostgresConnection};
use crate::queue_manager::manager::QueueManager;

pub struct ExecutionContext {
    pub auth_provider: Arc<dyn AuthProvider>,
    pub db: Arc<dyn DbConnectionPoolProvider<PoolType=Pool<Postgres>>>,
    pub env: Config,
    pub queue: mq_pool
}

impl ExecutionContext {
    pub async fn new_with_manager() -> Self {
        dotenv().ok();
        let config = Config::init();
        Self {
            auth_provider: Arc::new(AuthClient{}),
            db: Arc::new(PostgresConnection::init(&config.database_url).await),
            env: config.clone(),
            queue: QueueManager::init().await
        }
    }
}