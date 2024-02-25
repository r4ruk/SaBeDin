use amqprs::connection::{Connection, OpenConnectionArguments};
use axum::http::StatusCode;

pub async fn publish(_queue_name: &str, _object: String) -> Result<String, StatusCode>{
    let _addr = "amqp://raruk:raruk@localhost:5672";
    let _connection = Connection::open(&OpenConnectionArguments::new(
        "localhost",
        5672,
        "raruk",
        "test123"))
        .await
        .unwrap();



    Ok("Sent".to_string())
}