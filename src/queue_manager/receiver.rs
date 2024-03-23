use async_std::stream::StreamExt;
use lapin::{Channel, Consumer};
use lapin::options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use uuid::Uuid;
use crate::core::contracts::errors::GeneralServerError;
use crate::core::contracts::queue_types::{QueueRequestMessage, QueueResponseMessage};
use crate::queue_manager::manager::QueueManager;

/// receives a message from the given queue
pub async fn receive_on_queue(_manager: &QueueManager , channel: Channel, queue_name: &str)
    -> Result<QueueResponseMessage, GeneralServerError> {

    let mut consumer: Consumer = channel
        .basic_consume(
            queue_name,
            &channel.id().to_string(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let mut response_body= String::new();

    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery ) = delivery {
            println!("received msg: {:?}", delivery);
            response_body = String::from_utf8_lossy(&delivery.data).to_string();
            channel
                .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                .await?;

            let parsed:Result<QueueRequestMessage, GeneralServerError> =
                serde_json::from_str(&response_body)
                    .map_err(|e| GeneralServerError {
                        message: "could not map into QueueRequestMessage".to_string()});
            return match parsed {
                Ok(result) => { // valid delivery Request message containing correlation_id
                    Ok(QueueResponseMessage { correlation_id: result.correlation_id, body: response_body })
                }
                Err(_) => { // Error as it could not be parsed to our own type but still may be expected message from external service for example.
                    Ok(QueueResponseMessage { correlation_id: Uuid::new_v4(), body: response_body })
                }
            }
        }
    }
    return Err(GeneralServerError{ message: "could not read from queue".to_string() })
}

/// establishes a temporary listener
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