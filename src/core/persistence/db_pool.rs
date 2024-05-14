use std::sync::Arc;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;


// Define a trait for providing database connection pools
pub trait DbConnectionPoolProvider: Send + Sync {
    type PoolType;
    fn get_pool(&self) -> Arc<Self::PoolType>;
}

pub struct PostgresConnection {
    pool: Arc<Pool<Postgres>>,
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
                std::process::exit(1);
            }
        };
        return database_connection
    }
}

