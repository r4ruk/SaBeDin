use std::fmt::format;
use lapin::{BasicProperties, Channel};
use lapin::options::{BasicPublishOptions, QueueDeclareOptions};
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::contracts::base::queue_types::QueueRequestMessage;
use crate::logger::core_logger::{get_logger, LoggingLevel};
use crate::queue_manager::manager::QueueManager;

pub struct PublishParams<'a> {
    pub context: &'a ExecutionContext,
    pub manager: &'a QueueManager,
    pub queue_name: &'a str,
    pub channel: Channel,
    pub body: QueueRequestMessage
}

pub async fn basic_publish(params: PublishParams<'_>)
    -> Result<(), GeneralServerError> {
    let declare_options = QueueDeclareOptions {
        passive: false, // false -> defines a creation if it does not exist and otherwise returns existing, true does only work with existing
        durable: true, // delete on shutdown?
        exclusive: false, // exclusive for the given connection, after closing delete
        auto_delete: false, // auto delete when there is no consumer connected.
        nowait: true, // does not get a return from the queue -> fire and forget
    };
    let _creation_result = params.manager.create_queue(params.channel.clone(), params.queue_name, declare_options).await?;

    params.channel.basic_publish("", params.queue_name, BasicPublishOptions::default(), &serde_json::to_vec(&params.body).unwrap(), BasicProperties::default())
        .await
        .map_err(|e| {
            let logger = get_logger();
            logger.lock().unwrap().log_error(GeneralServerError{message:format!("could not publish: {}", e)}, LoggingLevel::Error);

            e
        })?
        .await
        .map_err(|e|{
            let logger = get_logger();
            logger.lock().unwrap().log_error(GeneralServerError{message:format!("could not publish: {}", e)}, LoggingLevel::Error);

            e
        })?;

    return Ok(())
}