use std::sync::Arc;
use lazy_static::lazy_static;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use crate::core::contracts::errors::GeneralServerError;
use std::env;
use crate::config::Config;

// Define a struct to hold the pool of connections
pub struct DbPool {
    pub pool: Pool<Postgres>,
}

lazy_static! {
    static ref POOL: Option<DbPool> = {
        let config = Config::init();
         let pool = match tokio::runtime::Runtime::new() {
            Ok(rt) => rt.block_on(DbPool::init(&config.database_url)),
            Err(_) => return None,
        };
        match pool {
            Ok(pool) => Some(pool),
            Err(_) => None,
        }
    };
}

impl DbPool {
    pub async fn init(database_url: &str) -> Result<Self, GeneralServerError> {
        let pool = match PgPoolOptions::new()
            .max_connections(90)
            .connect(database_url)
            .await
        {
            Ok(pool) => {
                println!("✅Connection to the database is successful!");
                pool
            }
            Err(err) => {
                println!("🔥 Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        };
        Ok(DbPool{pool})
    }
}

// Function to get a reference to the pool
pub fn get_db_pool() -> &'static Option<DbPool> {
    &POOL
}