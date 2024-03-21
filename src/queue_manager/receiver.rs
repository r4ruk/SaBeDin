use async_std::stream::StreamExt;
use lapin::{Channel, Consumer};
use lapin::options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use uuid::Uuid;
use crate::core::contracts::errors::GeneralServerError;
use crate::core::contracts::queue_types::QueueResponseMessage;
use crate::queue_manager::manager::{Connection, QueueManager, QueueManagerProvider};

pub async fn receive_on_queue(manager: &QueueManager , channel: Channel, queue_name: &str)
    -> Result<QueueResponseMessage, GeneralServerError> {
    // TODO implement basic receiver
        return Ok(QueueResponseMessage { correlation_id: Default::default(), body: "".to_string() })
}

pub(crate) async fn establish_temporary_listener(manager: &QueueManager , channel: Channel, correlation_id: Uuid)
                                      -> Result<QueueResponseMessage, GeneralServerError> {

    let queue_declareoptions = QueueDeclareOptions {
        passive: false,
        durable: false,
        exclusive: true,
        auto_delete: false,
        nowait: false,
    };

    let queue = manager.create_queue(channel.clone(), &correlation_id.to_string(),queue_declareoptions).await?;
    println!("Declared queue {:?}", queue);

    let mut consumer: Consumer = channel
        .basic_consume(
            &correlation_id.to_string(),
            &format!("{}",correlation_id),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let mut response_body= String::new();

    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            println!("received msg: {:?}", delivery);
            response_body = String::from_utf8_lossy(&delivery.data).to_string();
            channel
                .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                .await?;
            return Ok(QueueResponseMessage { correlation_id, body: response_body.to_string()})
        }
    }
    return Ok(QueueResponseMessage { correlation_id, body: response_body.to_string() })
}