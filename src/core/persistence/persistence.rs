use sqlx::migrate;

pub static MIGRATOR: sqlx::migrate::Migrator = migrate!("./src/migrations");