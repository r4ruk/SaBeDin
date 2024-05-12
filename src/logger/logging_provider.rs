use async_trait::async_trait;
use crate::core::contracts::errors::GeneralServerError;
use crate::logger::core_logger::LoggingLevel;

#[async_trait]
pub trait LoggingProvider: Send + Sync{
    async fn log_error(&self, err: GeneralServerError, logging_level: LoggingLevel) -> Result<(), GeneralServerError>;
}
