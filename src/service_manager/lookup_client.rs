use crate::core::contracts;
use crate::core::contracts::services::Service;

pub fn find_service(service_name: &str) -> Option<Box<dyn Service>>{
    match service_name {
        "client" => Some(contracts::core_services::client_service::ClientService::instantiate()),
        _ => None
    }
}