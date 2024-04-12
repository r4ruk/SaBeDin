use std::collections::HashMap;
use std::sync::Arc;
use crate::core::contracts::services::ClientHandler;
use tokio::sync::Mutex;
use async_trait::async_trait;
use log::{info, warn};
use uuid::Uuid;
use crate::core::contracts::basic_informations::{PagingQuery, RequestPostBody, ResponseBody};
use crate::core::contracts::dependency_container::ExecutionContext;
use crate::core::contracts::errors::GeneralServerError;
use crate::core::contracts::queue_types::QueueRequestMessage;
use crate::core::contracts::service_manager_provider::ServiceManagerProvider;
use crate::core::utils::{file_helper, utils};
use crate::queue_manager::manager::{QueueManager, QueueManagerProvider};
use crate::service_manager::service_client_factory;

#[async_trait]
pub trait ServiceManagerConstruction {
    async fn new() -> Self;
    async fn register_service(&mut self, service_name: String, service: Box<dyn ClientHandler>);
}

pub struct ServiceManager {
    pub services: Mutex<HashMap<String, Arc<Mutex<Box<dyn ClientHandler>>>>>
}

// implementation for the ServiceManagerExt trait which ensures the ServiceManager implements
// the try_handle functionality
#[async_trait]
impl ServiceManagerProvider for ServiceManager {
    async fn try_handle(&self, context: &ExecutionContext, path: &str, post_body: RequestPostBody) -> Result<(), GeneralServerError> {
        let binding = self.services.lock().await;
        let service_option = &binding.get(path);

        match service_option {
            Some(service) => {
                Ok(service.lock().await.handle_command(context, post_body).await)
            }
            None => {
                println!("no service found with name: {}", path);
                let queue = QueueManager{};
                queue.publish(context, path, QueueRequestMessage {
                    message_id: Uuid::new_v4(),
                    correlation_id: Uuid::new_v4(),
                    headers: path.to_string(),
                    body: post_body,
                    timestamp: Default::default(),
                }).await?;

                return Ok(())
            }
        }
    }

    async fn try_handle_query(&self, context: &ExecutionContext, service: &str, params: HashMap<String, String>) -> Result<ResponseBody, GeneralServerError> {
        let binding = self.services.lock().await; // using async lock
        let service_option = &binding.get(service);
        let mut response = ResponseBody{ body: "".to_string() };
        match service_option {
            Some(service) => {
                let serv = service.lock().await;
                response = serv.handle_query(context, params).await;
                Ok(response)
            }
            None => {
                println!("no service found with name: {:?}", service);
                println!("forwarding to queue with topic '{}' to handle it", service);
                let queue_manager = QueueManager { };
                let res = queue_manager.returning_publish(context, &service, QueueRequestMessage {
                    message_id: Uuid::new_v4(),
                    correlation_id: Default::default(),
                    headers: "".to_string(),
                    body: RequestPostBody {
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
impl ServiceManagerConstruction for ServiceManager {

    // instantiation of ServiceManager instance.
    async fn new() -> ServiceManager {
        let contents = file_helper::read_settings("config.setting");
        let os_specific_newline = utils::get_os_newline();

        let mut my_manager = ServiceManager {
            services: Mutex::new(Default::default())
        };
        match contents {
            Ok(content) => {
                // be aware to always end the config.setting file with an empty newline.
                // this ensures the correct functionality of the following code.
                if content.contains(&os_specific_newline.clone()) {
                    let lines = content.split(&os_specific_newline).collect::<Vec<&str>>();
                    for (_, line) in lines.iter().enumerate() {
                        if line != &"" {
                            info!("adding service {}", line);
                            // find Service implementation in service client factory
                            // and then register it in the manager
                            let client_option = service_client_factory::find_service(line);
                            match client_option {
                                Some(client) => {
                                    my_manager.register_service(line.to_string(), client).await;
                                },
                                None => {
                                    let interpolated = format!("Unknown type '{}' in factory.", line);
                                    println!("{}", interpolated)
                                }
                            }
                        }
                    }
                }
            },
            Err(e) => warn!("Could not read anything from the settings file. {:?}", e)
        }
        return my_manager
    }

    // registers the service in the Manager.
    async fn register_service(&mut self, service_name: String, service: Box<dyn ClientHandler>) {
        println!("Adding service with name: '{}'", service_name);
        self.services.lock().await.entry(service_name).or_insert(Arc::new(Mutex::new(service)));
    }
}