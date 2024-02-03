
#[cfg(test)]
mod service_manager_test {
    use crate::core::contracts::basic_informations::RequestBody;
    use crate::service_manager;
    use crate::service_manager::service_manager::IServiceManager;

    #[test]
    fn create_service_manager() {
        let manager = service_manager::service_manager::ServiceManager::new();
        let client_service = manager.services.get("Client");
        match client_service {
            Some(service) => service.handle_request(RequestBody{
                domain: "testdomain".to_string(),
                method: "get".to_string(),
                params: vec!["param1".to_string()] }),
            None => println!("not find client service")
        }
        assert_eq!(manager.services.len(), 1)

    }
}