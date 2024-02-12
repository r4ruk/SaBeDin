use std::collections::{HashSet};
use chrono::{DateTime, Utc};
use uuid::Uuid;


// TODO  read here about how to handle: https://blog.stackademic.com/rabbitmq-message-publisher-and-consumer-in-rust-9613113e89b2

trait IQueueManager {
    fn new() -> Self;
}

struct QueueManager {}

// body should be the expected services deserializable object structure representation in JSON
struct RequestMessage {
    message_id: Uuid,
    return_address: String,
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

impl IQueueManager for QueueManager {
    fn new() -> QueueManager {
        return QueueManager{  }
    }
}
