
#[cfg(test)]
mod service_manager_test {
    use std::collections::HashMap;
    use std::str::FromStr;
    use serde_json::json;
    use uuid::Uuid;
    use crate::core::contracts::base::basic_informations::RequestPostBody;
    use crate::core::contracts::dtos::user::User;
    use crate::service_manager::service_manager::{ServiceManagerConstruction, ServiceManager, SERVICE_MANAGER};
    use crate::core::contracts::traits::service_manager_provider::ServiceManagerProvider;
    use crate::core::persistence::core::db_pool::PostgresConnection;
    use crate::tests::common::test_helper::{create_execution_context_withdb, get_config};

    const SERVICE_NAME: &str = "user";


    #[tokio::test]
    async fn create_service_manager() {
        let count_of_services = SERVICE_MANAGER.services.read().await.iter().count();
        assert_eq!(count_of_services, 1);
    }

    // ATTENTION THIS TEST NEEDS CONTAINER WITH DB RUNNING
    #[tokio::test]
    async fn handle_query_test() {

        let mut param: HashMap<String, String> = HashMap::new();
        param.insert("email".to_string(), "hans.ueli@test.ch".to_string());
        let db = PostgresConnection::init(&get_config().database_url).await;
        let mq = crate::queue_manager::manager::QueueManager::init().await;
        let returned_user = SERVICE_MANAGER.try_handle_query(&create_execution_context_withdb(db, mq, None).await, SERVICE_NAME, param).await.unwrap();
        let user_object: User = serde_json::from_str(&returned_user.body).unwrap();

        assert_eq!(user_object.email, "hans.ueli@test.ch".to_string());
        assert_eq!(user_object.password, "superSafePassword".to_string());
    }

    // ATTENTION THIS TEST NEEDS CONTAINER WITH DB RUNNING
    #[tokio::test]
    async fn handle_command_test() {
        let db = PostgresConnection::init(&get_config().database_url).await;
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
            idempotency_key: Uuid::new_v4().to_string(),
            method: "generalmethod".to_string(),
            object: json!(user).to_string(),
            params: HashMap::new(),
            query_options: Default::default(),
        };
        let rs = &create_execution_context_withdb(db, mq, None).await;
        let _r = SERVICE_MANAGER.try_handle_command(rs, SERVICE_NAME, requestpostbody).await;
        println!("test successfully run")
    }
}