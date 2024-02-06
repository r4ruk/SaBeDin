use std::collections::{HashSet};
use chrono::{DateTime, Utc};
use kafka::client::KafkaClient;
use log::{error, info, warn};
use crate::core::contracts::file_helper;
use crate::service_manager::lookup_client;

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

        // TODO refine this section
        let topics = file_helper::read_settings("queue_topics.settings");
        match topics {
            Ok(content) => {
                if content.contains("\n") {
                    let lines = content.split('\n').collect::<Vec<&str>>();
                    for (_, line) in lines.iter().enumerate(){
                        info!("adding topic {}", line);
                        if !manager.topics.contains(*line) {
                            // TODO Add topic to Kafka if it does not exist yet
                        } else {
                            // TODO already exists. do something...?
                        }
                    }
                }
            },
            Err(e) => warn!("Could not read topics from the settings file. {:?}", e)
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