use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::core::contracts::base::basic_informations::{CommandResponse, RequestPostBody, ResponseBody};
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::contracts::base::queue_types::QueueRequestMessage;
use crate::core::contracts::base::system_messages::InformationMessage;
use crate::core::contracts::traits::service_manager_provider::{ExtendableServiceManager, ExtendableServiceManagerProvider, ServiceManagerProvider};
use crate::core::contracts::traits::services::ClientHandler;
use crate::core::utils::{file_helper, utils};
use crate::logger::core_logger::{get_logger, LoggingLevel};
use crate::queue_manager::manager::{QueueManager, QueueManagerProvider};
use crate::service_manager::service_client_factory;


pub struct GlobalServiceManager {
    pub services: RwLock<HashMap<String, Arc<Box<dyn ClientHandler>>>>,
    pub external_services: RwLock<Vec<String>>
}

lazy_static! {
  pub static ref SERVICE_MANAGER: GlobalServiceManager = GlobalServiceManager::new();
}

impl ExtendableServiceManagerProvider for GlobalServiceManager{}

// implementation for the ServiceManagerExt trait which ensures the ServiceManager implements
#[async_trait]
impl ServiceManagerProvider for GlobalServiceManager {
    async fn try_handle_command(&self, context: &ExecutionContext, path: &str, post_body: RequestPostBody) -> Result<CommandResponse, GeneralServerError> {
        let binding = self.services.read().await;
        let service_option = binding.get(path);

        match service_option {
            Some(service) => {
                let result = service.handle_command(context, post_body).await?;
                Ok(CommandResponse { code: 200, response: result.body })
            }
            None => {
                let logger = get_logger();
                logger.lock().unwrap().log_message(GeneralServerError{message:format!("no service found with name: {}", path)}, LoggingLevel::Information);

                if !SERVICE_MANAGER.external_services.read().await.contains(&path.to_string()) {
                    let err = GeneralServerError{message:format!("queue for external service not registered: '{}'", path)};
                    logger.lock().unwrap().log_message(err.clone(), LoggingLevel::Error);
                    return Err(err)
                }

                let queue = QueueManager{};
                queue.publish(context, path, QueueRequestMessage {
                    message_id: Uuid::new_v4(),
                    correlation_id: Uuid::new_v4(),
                    headers: path.to_string(),
                    body: post_body,
                    timestamp: Default::default(),
                }).await?;

                return Ok(CommandResponse { code: 200, response: "didnt handle anything.".to_string() })
            }
        }
    }

    async fn try_handle_query(&self, context: &ExecutionContext, service: &str, params: HashMap<String, String>) -> Result<ResponseBody, GeneralServerError> {
        let binding = SERVICE_MANAGER.services.read().await;

        println!("Current services: {:?}", binding.keys().collect::<Vec<_>>());
        let service_option = binding.get(service);
        match service_option {
            Some(service) => {
                let response = service.handle_query(context, params).await;
                Ok(response)
            }
            None => {
                let logger = get_logger();
                logger.lock().unwrap().log_message(GeneralServerError{message:format!("no service found with name: {:?}", service)}, LoggingLevel::Error);


                logger.lock().unwrap().log_message(GeneralServerError{message:format!("forwarding to queue with topic '{}' to handle it", service)}, LoggingLevel::Information);

                let queue_manager = QueueManager { };
                let res = queue_manager.returning_publish(context, &service, QueueRequestMessage {
                    message_id: Uuid::new_v4(),
                    correlation_id: Default::default(),
                    headers: "".to_string(),
                    body: RequestPostBody {
                        idempotency_key: Default::default(),
                        method: "get".to_string(),
                        object: "".to_string(),
                        params,
                        query_options: Default::default(),
                    },
                    timestamp: Default::default(),
                }).await?;
                Ok(ResponseBody{body: res.body})
            }
        }
    }
}

#[async_trait]
impl ExtendableServiceManager for GlobalServiceManager {
    // registers services for external applications / microservices which are allowed in queue
    async fn register_external_service(&mut self, service_name: String) {
        let logger = get_logger();
        logger.lock().unwrap().log_message(InformationMessage{message:format!("Adding external service with name: '{}'", service_name)}, LoggingLevel::Information);

        if !self.external_services.read().await.contains(&service_name.clone()) {
            self.external_services.write().await.push(service_name);
        }
    }
}

impl GlobalServiceManager {

    // instantiation of ServiceManager instance.
    fn new() -> GlobalServiceManager {
        let contents = file_helper::read_settings("config.setting");
        let os_specific_newline = utils::get_os_newline();

        let my_manager = GlobalServiceManager {
            services: Default::default(),
            external_services: Default::default(),
        };
        match contents {
            Ok(content) => {
                // be aware to always end the config.setting file with an empty newline.
                // this ensures the correct functionality of the following code.
                if content.contains(&os_specific_newline.clone()) {
                    let lines = content.split(&os_specific_newline).collect::<Vec<&str>>();
                    for (_, line) in lines.iter().enumerate() {
                        if line != &"" {
                            let logger = get_logger();
                            logger.lock().unwrap().log_message(InformationMessage{message:format!("Trying to add service {}", line)}, LoggingLevel::Information);
                            // find Service implementation in service client factory
                            // and then register it in the manager
                            let client_option = service_client_factory::find_service(line);
                            match client_option {
                                Some(client) => {
                                    futures::executor::block_on(async {
                                        my_manager.register_service(line.to_string(), client).await
                                    })
                                },
                                None => {
                                    let logger = get_logger();
                                    logger.lock().unwrap().log_message(InformationMessage{message:format!("Unknown type '{}' in factory.", line)}, LoggingLevel::Information);
                                }
                            }
                        }
                    }
                }
            },
            Err(e) => {
                let logger = get_logger();
                logger.lock().unwrap().log_message(InformationMessage{message:format!("Could not read anything from the settings file. {:?}", e)}, LoggingLevel::Warning);
            }
        }
        futures::executor::block_on(async {
            let logger = get_logger();
            logger.lock().unwrap().log_message(InformationMessage { message: format!("my manager after initialization: {:?}", my_manager.services.read().await.keys().collect::<Vec<_>>())}, LoggingLevel::Information);
        });
        return my_manager
    }

    // registers the service in the Manager.
    pub(crate) async fn register_service(&self, service_name: String, service: Box<dyn ClientHandler>) {

        let logger = get_logger();
        logger.lock().unwrap().log_message(InformationMessage{message:format!("Adding service with name: '{}'", service_name)}, LoggingLevel::Information);

        self.services.write().await.entry(service_name).or_insert(Arc::new(service));
    }
}
