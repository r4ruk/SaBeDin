use std::collections::{HashSet};
use chrono::{DateTime, Utc};
use kafka::client::KafkaClient;


trait IQueueManager {
    fn new() -> Self;
    fn check_topic_available(&self, name: String) -> bool;
}

struct QueueManager {
    topics: HashSet<String>,
}

struct Message {
    action: String,
    content: String,
    timestamp: DateTime<Utc>,
    metadata: String
}

impl IQueueManager for QueueManager {
    fn new() -> QueueManager {
        let hosts:Vec<String> = vec!["localhost:9092".to_string()];
        let mut manager = QueueManager { topics: HashSet::new() };

        let mut client = KafkaClient::new(hosts);
        client.load_metadata_all().unwrap();
        for topic in client.topics() {
            if !manager.topics.contains(topic.name()) {
                manager.topics.insert(topic.name().to_string());
            }
        }
        manager
    }

    fn check_topic_available(&self, name: String) -> bool {
        return match self.topics.get(&name) {
            Some(_) => true,
            None => false
        }
    }
}