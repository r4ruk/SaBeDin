use std::sync::Arc;
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use deadpool_lapin::Pool as mq_pool;
use crate::config::Config;
use crate::core::client::auth::AuthClient;
use crate::core::contracts::dependency_container::ExecutionContext;
use crate::service_manager::service_manager::{IServiceManager, ServiceManager};

#[allow(unused)]
pub fn get_config() -> Config {
    dotenv().ok();
    let config = Config::init();
    return config;
}

#[allow(unused)]
pub async fn create_execution_context(db: Pool<Postgres>, qm: mq_pool, config: Option<Config>) -> ExecutionContext {
    return match config {
        Some(conf) => ExecutionContext {
            service_manager: Arc::new(ServiceManager::new().await).clone(),
            auth_provider: Arc::new(AuthClient{}),
            env: conf.clone(),
            db,
            queue: qm,
        },
        None => ExecutionContext {
            service_manager: Arc::new(ServiceManager::new().await).clone(),
            auth_provider: Arc::new(AuthClient{}),
            env: get_config().clone(),
            db,
            queue: qm,
        }
    };
}