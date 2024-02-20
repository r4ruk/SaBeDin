
#[cfg(test)]
mod service_manager_test {
    use std::collections::HashMap;
    use crate::core::contracts::basic_informations::{RequestPostBody, ResponseBody};
    use crate::core::contracts::user::User;
    use crate::service_manager::service_manager::{IServiceManager, ServiceManager, ServiceManagerExt};
    const SERVICE_NAME: &str = "client";


    #[test]
    fn create_service_manager() {
        let manager = ServiceManager::new();
        let count_of_services = manager.services.lock().unwrap().iter().count();
        assert_eq!(count_of_services, 1);
    }

    #[test]
    fn handle_query_test() {
        let manager = ServiceManager::new();

        let mut param: HashMap<String, String> = HashMap::new();
        param.insert("id".to_string(), "1".to_string());

        let returned_user:  ResponseBody = manager.try_handle_query(SERVICE_NAME.to_string(), param);
        let user_object: User = serde_json::from_str(&returned_user.body).unwrap();

        assert_eq!(user_object.email, "hans.ueli@test.ch".to_string());
        assert_eq!(user_object.password, "superSafePassword".to_string());
    }

    #[test]
    fn handle_command_test() {
        let manager = ServiceManager::new();
        let requestpostbody = RequestPostBody {
            method: "generalmethod".to_string(),
            object: "{\"id\":\"0f083f37-0693-42b8-8a3e-6b1dfa0221ff\",\"name\":\"John Doe\",\"password\":\"password123\",\"email\":\"john@example.com\",\"age\":30}".to_string(),
            params: vec!["1".to_string()],
        };
        let r = manager.try_handle(SERVICE_NAME.to_string(), requestpostbody);
        println!("test successfully run")
    }
}