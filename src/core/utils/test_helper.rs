use std::sync::Arc;
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use deadpool_lapin::Pool as mq_pool;
use crate::config::Config;
use crate::core::client::auth::AuthClient;
use crate::core::contracts::dependency_container::ExecutionContext;
use crate::core::persistence::db_pool::DbConnectionPoolProvider;
use crate::service_manager::service_manager::{ServiceManagerConstruction, ServiceManager};

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
            db: Arc::new(MockDbConnectionPoolProvider{}),
            queue: qm,
        },
        None => ExecutionContext {
            service_manager: Arc::new(ServiceManager::new().await).clone(),
            auth_provider: Arc::new(AuthClient{}),
            env: get_config().clone(),
            db: Arc::new(MockDbConnectionPoolProvider{}),
            queue: qm,
        }
    };
}


// TODO think about central place to put mocks. not good here.
// Define a mock implementation of DbConnectionPoolProvider for testing
pub struct MockDbConnectionPoolProvider;

impl DbConnectionPoolProvider for MockDbConnectionPoolProvider {
    type PoolType = Pool<Postgres>;

    fn get_pool(&self) -> Arc<Self::PoolType> {
        // Return a mock pool wrapped in an Arc
        unimplemented!("asdf")
    }
}