use crate::core::client;
use crate::core::contracts::services::ClientHandler;

// function used to register Services, so it can be found from ServiceManager to further route requests
// to project internal services.
pub fn find_service(service_name: &str) -> Option<Box<dyn ClientHandler>>{
    match service_name {
        "client" => Some(client::user::UserClient::instantiate()),
        _ => None
    }
}