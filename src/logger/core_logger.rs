use std::sync::{Arc, Mutex};
use crate::core::contracts::errors::GeneralServerError;
use crate::name_of;

#[allow(unused)]
pub enum LoggingLevel{
    Information,
    Warning,
    Error
}

pub struct Logger {}


impl Logger {
    pub fn log_error(&self, err: GeneralServerError, logging_level: LoggingLevel) {
        println!("error logged: {}, logging_level: '{}'", err.message, name_of!(logging_level));
    }
}

// Define a static instance of the logger
lazy_static::lazy_static! {
    static ref LOGGER: Arc<Mutex<Logger>> = Arc::new(Mutex::new(Logger {}));
}

// Function to get the logger instance
pub fn get_logger() -> Arc<Mutex<Logger>> {
    LOGGER.clone()
}