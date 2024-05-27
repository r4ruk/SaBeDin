use std::sync::Arc;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::logger::core_logger::{get_logger, LoggingLevel};

pub struct PostgresConnection {
    pool: Arc<Pool<Postgres>>,
}

// Define a trait for providing database connection pools
pub trait DbConnectionPoolProvider: Send + Sync {
    type PoolType;
    fn get_pool(&self) -> Arc<Self::PoolType>;
}

impl DbConnectionPoolProvider for PostgresConnection {
    type PoolType = Pool<Postgres>;
    fn get_pool(&self) -> Arc<Self::PoolType>  {
        return Arc::clone(&self.pool)
    }
}

impl PostgresConnection {
    pub async fn init(database_url: &str) -> PostgresConnection {
        let database_connection = match PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await
        {
            Ok(pool) => {
                println!("✅ Connection to the database is successful!");
                Self {
                    pool: Arc::new(pool)
                }
            }
            Err(err) => {
                println!("🔥 Failed to connect to the database: {:?}", err);

                let err = GeneralServerError { message: format!("failed to connect to database: {}",err ) };
                let logger = get_logger();
                logger.lock().unwrap().log_error(err.clone(), LoggingLevel::Error);

                std::process::exit(1);
            }
        };
        return database_connection
    }
}

