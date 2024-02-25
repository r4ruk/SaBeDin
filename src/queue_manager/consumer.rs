use axum::http::StatusCode;

pub async fn consumer(_topic: &str) -> Result<String,StatusCode> {
    let _addr = "amqp://raruk:raruk@localhost:5672";

    return Ok("success".to_string())
}

