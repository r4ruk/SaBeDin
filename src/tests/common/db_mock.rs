use std::sync::Arc;
use sqlx::{Pool, Postgres};
use crate::core::persistence::db_pool::DbConnectionPoolProvider;

/// Mock implementation for DbConnectionPoolProvider
pub struct MockDbConnectionPoolProvider;

impl DbConnectionPoolProvider for MockDbConnectionPoolProvider {
    type PoolType = Pool<Postgres>;

    fn get_pool(&self) -> Arc<Self::PoolType> {
        // Return a mock pool wrapped in an Arc
        unimplemented!("asdf")
    }
}