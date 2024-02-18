use amqprs::connection::{Connection, OpenConnectionArguments};
use axum::http::StatusCode;

pub async fn publish(queue_name: &str, object: String) -> Result<String, StatusCode>{
    let addr = "amqp://raruk:raruk@localhost:5672";
    let connection = Connection::open(&OpenConnectionArguments::new(
        "localhost",
        5672,
        "raruk",
        "test123"))
        .await
        .unwrap();



    Ok("Sent".to_string())
}