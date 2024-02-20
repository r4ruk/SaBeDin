#[cfg(test)]
mod lookup_client_test {
    use super::super::lookup_client;
    use std::any::{Any, TypeId};
    use crate::core::client::user::UserClient;
    use crate::core::contracts::services::ClientHandler;

    // IMPORTANT add new services here as soon as new ones are added
    const KNOWN_SERVICES: [(&str, fn() -> Box<dyn ClientHandler>); 1] = [
        ("client", || UserClient::instantiate()),
    ];

    #[test]
    fn retrieve_specific_clients() {
        for service_name in KNOWN_SERVICES {
            let service = lookup_client::find_service(service_name.0);
            assert!(service.is_some());

            let service_box:Box<dyn ClientHandler> = service.unwrap();
            let my_type = service_name.1();

            assert_eq!(service_box.as_ref().type_id(), my_type.as_ref().type_id());
        }
    }
}