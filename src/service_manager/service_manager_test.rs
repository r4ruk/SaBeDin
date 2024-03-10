
#[cfg(test)]
mod service_manager_test {
    use std::collections::HashMap;
    use std::str::FromStr;
    use serde_json::json;
    use uuid::Uuid;
    use crate::core::contracts::basic_informations::{RequestPostBody, ResponseBody};
    use crate::core::contracts::user::User;
    use crate::service_manager::service_manager::{IServiceManager, ServiceManager};
    use crate::core::contracts::service_manager_provider::ServiceManagerProvider;
    use crate::core::utils::test_helper::{create_execution_context, get_config};

    const SERVICE_NAME: &str = "client";


    #[tokio::test]
    async fn create_service_manager() {
        let manager = ServiceManager::new().await;
        let count_of_services = manager.services.lock().await.iter().count();
        assert_eq!(count_of_services, 1);
    }

    #[tokio::test]
    async fn handle_query_test() {
        let manager = ServiceManager::new().await;

        let mut param: HashMap<String, String> = HashMap::new();
        param.insert("id".to_string(), "1".to_string());
        let db = crate::core::persistence::db_pool::init(&get_config().database_url).await;
        let mq = crate::queue_manager::manager::QueueManager::init().await;
        let returned_user = manager.try_handle_query(&create_execution_context(db, mq, None).await, SERVICE_NAME.to_string(), param).await.unwrap();
        let user_object: User = serde_json::from_str(&returned_user.body).unwrap();

        assert_eq!(user_object.email, "hans.ueli@test.ch".to_string());
        assert_eq!(user_object.password, "superSafePassword".to_string());
    }

    #[tokio::test]
    async fn handle_command_test() {
        let manager = ServiceManager::new().await;
        let db = crate::core::persistence::db_pool::init(&get_config().database_url).await;
        let mq = crate::queue_manager::manager::QueueManager::init().await;
        let user = User{
            id: Uuid::from_str("0f083f37-0693-42b8-8a3e-6b1dfa0221ff").unwrap(),
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            password: "password123".to_string(),
            role: "user".to_string(),
            verified: false,
            created_at: Default::default(),
            updated_at: Default::default(),
        };

        let requestpostbody = RequestPostBody {
            method: "generalmethod".to_string(),
            object: json!(user).to_string(),
            params: HashMap::new(),
        };
        let r = manager.try_handle(&create_execution_context(db, mq, None).await, SERVICE_NAME.to_string(), requestpostbody).await;
        println!("test successfully run")
    }
}