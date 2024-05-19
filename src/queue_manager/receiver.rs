use async_std::stream::StreamExt;
use axum::routing::get;
use lapin::{Channel, Consumer};
use lapin::options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use uuid::Uuid;
use crate::core::contracts::errors::GeneralServerError;
use crate::core::contracts::queue_types::{QueueRequestMessage, QueueResponseMessage};
use crate::core::contracts::system_messages::InformationMessage;
use crate::logger::core_logger::{get_logger, LoggingLevel};
use crate::queue_manager::manager::QueueManager;
use tokio::time::{timeout, Duration};

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

            // TODO really want to log or print here or maybe better even creating event...???
            println!("received msg: {:?}", delivery);
            response_body = String::from_utf8_lossy(&delivery.data).to_string();
            channel
                .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                .await?;

            let parsed:Result<QueueRequestMessage, GeneralServerError> =
                serde_json::from_str(&response_body)
                    .map_err(|_| GeneralServerError {
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

    let timeout_duration = Duration::from_secs(15);

    let queue_declareoptions = QueueDeclareOptions {
        passive: false,
        durable: false,
        exclusive: true,
        auto_delete: false,
        nowait: false,
    };

    let queue = manager.create_queue(channel.clone(), &correlation_id.to_string(),queue_declareoptions).await?;
    
    let logger = get_logger();
    logger.lock().unwrap().log_error(InformationMessage { message: format!("Declared queue {:?}", queue) }, LoggingLevel::Information);

    let mut consumer: Consumer = channel
        .basic_consume(
            &correlation_id.to_string(),
            &format!("{}",correlation_id),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let result = timeout(timeout_duration, async {
        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                println!("received msg: {:?}", delivery);
                let response_body = String::from_utf8_lossy(&delivery.data).to_string();
                channel
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await?;
                return Ok(QueueResponseMessage {
                    correlation_id: correlation_id.clone(),
                    body: response_body,
                });
            }
        }
        // Return an error if the loop completes without receiving a message
        Err(GeneralServerError {
            message: "No message received".into(),
        })
    }).await;

    let ret_result = match result {
        // all good, return response mapped inside retrieval function
        Ok(Ok(response)) => Ok(response),

        // Error happened when message was received...
        Ok(Err(err)) =>{
            let logger = get_logger();
            logger.lock().unwrap().log_error(GeneralServerError{
                message: format!("Internal error in queue message retrieval for correlationId: {}", correlation_id)
            }, LoggingLevel::Error);
            Err(err)
        },

        // Connection timeout reached so returning unsuccessful
        Err(_) => {
            let logger = get_logger();
            logger.lock().unwrap().log_error(GeneralServerError{
                message: format!("Timeout in queue message retrieval for correlationId: {}", correlation_id)
            }, LoggingLevel::Error);

            Err(GeneralServerError {
            message: "Timeout elapsed".into(),
            })
        },
    };
    return ret_result
}