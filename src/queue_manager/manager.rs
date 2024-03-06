use async_trait::async_trait;
use chrono::{DateTime, Utc};
use lapin::{Connection, ConnectionProperties};
use uuid::Uuid;
use crate::core::contracts::basic_informations::ResponseBody;
use crate::core::contracts::errors::GeneralServerError;

#[async_trait]
trait IQueueManager {
    async fn new() -> QueueManager;
    async fn responding_publish(entity: &str, content: &str) -> Result<ResponseBody, GeneralServerError>;
}

struct QueueManager {}

// body should be the expected services deserializable object structure representation in JSON
struct RequestMessage {
    message_id: Uuid,
    headers: String,
    body: String,
    timestamp: DateTime<Utc>,
}

// body should be the expected services deserializable object structure representation in JSON
struct ResponseMessage {
    correlation_id: Uuid,
    body: String
}

// communication should be:
// client -> sends request to channel and provides return address channel
// -> Service handles request (probably only get requests...?!) -> returns ResponseMessage to return_channel
// correlation_id should be the same as message_id of the request, so it knows it's reading the right message.

#[async_trait]
impl IQueueManager for QueueManager {
    async fn new() -> QueueManager {
        // let connection = Connection::open(&OpenConnectionArguments::new(
        //     "localhost",
        //     5672,
        //     "raruk",
        //     "test123"))
        //     .await
        //     .unwrap();
        let addr = "amqp://raruk:test123@127.0.0.1:5672";
        let conn = Connection::connect(addr, ConnectionProperties::default()).await.map_err(|_| println!("error happened") );

        QueueManager {
        }
    }

    async fn responding_publish(entity: &str, content: &str) -> Result<ResponseBody, GeneralServerError> {





        return Ok(ResponseBody{ body: "".to_string() })
    }
}
