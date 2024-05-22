use std::sync::{Arc, Mutex};
use crate::core::contracts::system_messages::SysMessage;
use crate::name_of;

#[allow(unused)]
#[derive(Debug)]
pub enum LoggingLevel{
    Information,
    Warning,
    Error
}

pub struct Logger {}

impl Logger {
    pub fn log_error<T:SysMessage>(&self, err: T, logging_level: LoggingLevel) {
        println!("Logged: {}, logging_level: '{:?}'", err.get_internal_message(), logging_level);
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