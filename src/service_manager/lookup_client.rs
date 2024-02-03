use crate::core::contracts;
use crate::core::contracts::services::Service;

pub fn find_service(service_name: &str) -> Option<Box<dyn Service>>{
    match service_name {
        "Client" => Some(contracts::core_services::client_service::Client::instantiate()),
        _ => None
    }
}