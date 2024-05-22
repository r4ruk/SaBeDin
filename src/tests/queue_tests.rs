#[cfg(test)]
mod queue_tests {
    use uuid::Uuid;
    use crate::core::contracts::basic_informations::RequestPostBody;
    use crate::core::contracts::queue_types::QueueRequestMessage;
    use crate::queue_manager::manager::{QueueManager, QueueManagerProvider};
    use crate::tests::common::test_helper::create_execution_context;

    #[tokio::test]
    async fn manager_basic_publish_test() {
        let mq = QueueManager::init().await;


        let context = create_execution_context(mq, None).await;
        let queue_manager = QueueManager {};
        let res = queue_manager.publish(&context, "test", QueueRequestMessage {
            message_id: Uuid::new_v4(),
            correlation_id: Default::default(),
            headers: "".to_string(),
            body: RequestPostBody {
                idempotency_key: Uuid::new_v4().to_string(),
                method: "get".to_string(),
                object: "".to_string(),
                params: Default::default(),
                query_options: Default::default(),
            },
            timestamp: Default::default(),
        }).await;

        println!("{:?}", res);

        // let res = queue_manager.returning_publish(&context, "test", QueueRequestMessage {
        //     message_id: Uuid::new_v4(),
        //     headers: "".to_string(),
        //     body: RequestPostBody {
        //         method: "get".to_string(),
        //         object: "".to_string(),
        //         params,
        //     },
        //     timestamp: Default::default(),
        // }).await;
    }

    // This test is manually run with publishing message onto the queue by hand.
    #[tokio::test]
    async fn manager_returning_publish_test() {

        let mq = QueueManager::init().await;

        let correlation_id = Uuid::new_v4();

        let context = create_execution_context(mq, None).await;
        let queue_manager = QueueManager { };
        let res = queue_manager.returning_publish(&context, "test", QueueRequestMessage {
            message_id: Uuid::new_v4(),
            correlation_id,
            headers: "".to_string(),
            body: RequestPostBody {
                idempotency_key: Uuid::new_v4().to_string(),
                method: "get".to_string(),
                object: "".to_string(),
                params: Default::default(),
                query_options: Default::default(),
            },
            timestamp: Default::default(),
        }).await;

        assert!(res.is_ok())
    }

    #[tokio::test]
    async fn read_write_test() {
        let mq = QueueManager::init().await;


        let context = create_execution_context(mq, None).await;
        let queue_manager = QueueManager {};
        let correlation_id = Uuid::new_v4();
        let res = queue_manager.publish(&context, "test", QueueRequestMessage {
            message_id: Uuid::new_v4(),
            correlation_id,
            headers: "".to_string(),
            body: RequestPostBody {
                idempotency_key: Uuid::new_v4().to_string(),
                method: "get".to_string(),
                object: "".to_string(),
                params: Default::default(),
                query_options: Default::default(),
            },
            timestamp: Default::default(),
        }).await;

        let result = queue_manager.get_from_queue(&context , "test").await;
        println!("{} equals {}", &correlation_id.to_string(), &result.as_ref().unwrap().correlation_id.to_string());
        assert_eq!(correlation_id, result.unwrap().correlation_id);
    }

}