use sqlx::migrate;

#[allow(unused)]
pub static MIGRATOR: sqlx::migrate::Migrator = migrate!("./src/migrations");