use uuid::Uuid;
use crate::core::contracts::basic_informations::RequestPostBody;
use crate::core::contracts::queue_types::QueueRequestMessage;
use crate::core::utils::test_helper;
use crate::core::utils::test_helper::get_config;
use crate::queue_manager::manager::{QueueManager, QueueManagerProvider};

#[tokio::test]
async fn manager_basic_publish_test() {

    let db = crate::core::persistence::db_pool::init(&get_config().database_url).await;
    let mq = crate::queue_manager::manager::QueueManager::init().await;


    let context = test_helper::create_execution_context(db, mq, None).await;
    let queue_manager = QueueManager { };
    let res = queue_manager.basic_publish(&context, "test", QueueRequestMessage {
        message_id: Uuid::new_v4(),
        headers: "".to_string(),
        body: RequestPostBody {
            method: "get".to_string(),
            object: "".to_string(),
            params: Default::default(),
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

// #[tokio::test]
// async fn manager_returning_publish_test() {
//     let db = crate::core::persistence::db_pool::init(&get_config().database_url).await;
//     let mq = QueueManager::init().await;
//
//
//     let context = test_helper::create_execution_context(db, mq, None).await;
//     let queue_manager = QueueManager { };
//     let res = queue_manager.returning_publish(&context, "test", QueueRequestMessage {
//         message_id: Uuid::new_v4(),
//         headers: "".to_string(),
//         body: RequestPostBody {
//             method: "get".to_string(),
//             object: "".to_string(),
//             params,
//         },
//         timestamp: Default::default(),
//     }).await;
//     assert!(res.is_ok())
// }