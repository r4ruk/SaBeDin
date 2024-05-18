use std::collections::HashMap;
use crate::core::contracts::errors::GeneralServerError;
use crate::logger::core_logger::{get_logger, LoggingLevel};

// function handles different params arriving from GET request
pub fn handle_params(params: &str) -> HashMap<String, String> {
    let mut map_params: HashMap<String, String>  = HashMap::new();
    let param_vec:Vec<&str> = params.split('&').collect::<Vec<&str>>();
    for param in param_vec {
        if let Some((name, value)) = param.split_once('=') {
            map_params.entry(name.to_string().to_lowercase()).or_insert(value.to_string());
        } else {
            let logger = get_logger();
            logger.lock().unwrap().log_error(GeneralServerError{message:"could not read name value params".into()}, LoggingLevel::Error);
        }
    }
    return map_params
}