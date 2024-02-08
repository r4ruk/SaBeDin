use std::collections::{HashSet};
use chrono::{DateTime, Utc};

trait IQueueManager {
    fn new() -> Self;
}

struct QueueManager {
    topics: HashSet<String>,
}

// body should be the expected services deserializable object structure representation in JSON
struct RequestMessage {
    message_id: uuid,
    return_address: String,
    headers: String,
    body: String,
    timestamp: DateTime<Utc>,
}

// body should be the expected services deserializable object structure representation in JSON
struct ResponseMessage {
    correlation_id: uuid,
    body: String
}

// communication should be:
// client -> sends request to channel and provides return address channel
// -> Service handles request (probably only get requests...?!) -> returns ResponseMessage to return_channel
// correlation_id should be the same as message_id of the request, so it knows it's reading the right message.

impl IQueueManager for QueueManager {
    fn new() -> QueueManager {
        return QueueManager{ topics: Default::default() }
    }
}
