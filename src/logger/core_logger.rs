use async_trait::async_trait;
use crate::core::contracts::errors::GeneralServerError;
use crate::logger::logging_provider::LoggingProvider;
use crate::name_of;

#[allow(unused)]
pub enum LoggingLevel{
    Information,
    Warning,
    Error
}

pub struct CoreLogger {}

#[async_trait]
impl LoggingProvider for CoreLogger {
    async fn log_error(&self, err: GeneralServerError, logging_level: LoggingLevel) -> Result<(), GeneralServerError> {
        println!("error logged: {}, logging_level: '{}'", err.message, name_of!(logging_level));
        return Ok(())
    }
}

impl CoreLogger{
    pub fn initialize() -> Self {
        Self{}
    }
}