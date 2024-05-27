use std::sync::Arc;
use dotenv::dotenv;
use deadpool_lapin::Pool as mq_pool;
use crate::config::Config;
use crate::core::client::auth::AuthClient;
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::persistence::core::db_pool::PostgresConnection;
use crate::service_manager::service_manager::{ServiceManagerConstruction, ServiceManager};

use super::*;

#[allow(unused)]
pub fn get_config() -> Config {
    dotenv().ok();
    let config = Config::init();
    return config;
}

#[allow(unused)]
pub async fn create_execution_context(qm: mq_pool, config: Option<Config>) -> ExecutionContext {
    return match config {
        Some(conf) => ExecutionContext {
            service_manager: Arc::new(ServiceManager::new().await).clone(),
            auth_provider: Arc::new(AuthClient{}),
            env: conf.clone(),
            db: Arc::new(db_mock::MockDbConnectionPoolProvider{}),
            queue: qm,
        },
        None => ExecutionContext {
            service_manager: Arc::new(ServiceManager::new().await).clone(),
            auth_provider: Arc::new(AuthClient{}),
            env: get_config().clone(),
            db: Arc::new(db_mock::MockDbConnectionPoolProvider{}),
            queue: qm,
        }
    };
}

#[allow(unused)]
pub async fn create_execution_context_withdb(db: PostgresConnection,qm: mq_pool, config: Option<Config>) -> ExecutionContext {
    return match config {
        Some(conf) => ExecutionContext {
            service_manager: Arc::new(ServiceManager::new().await).clone(),
            auth_provider: Arc::new(AuthClient{}),
            env: conf.clone(),
            db: Arc::new(db),
            queue: qm,
        },
        None => ExecutionContext {
            service_manager: Arc::new(ServiceManager::new().await).clone(),
            auth_provider: Arc::new(AuthClient{}),
            env: get_config().clone(),
            db: Arc::new(db),
            queue: qm,
        }
    };
}
