use axum::http::StatusCode;

const QUEUE_NAME: &str = "hello";

pub async fn consumer(topic: &str) -> Result<String,StatusCode> {
    let addr = "amqp://raruk:raruk@localhost:5672";

    return Ok("success".to_string())
}

