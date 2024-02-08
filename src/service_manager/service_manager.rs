use std::collections::HashMap;
use crate::core::contracts::services::Service;
use std::sync::{Arc, Mutex};
use log::{error, info, warn};
use crate::core::contracts::basic_informations::RequestPostBody;
use crate::core::contracts::file_helper;
use crate::service_manager::lookup_client;


pub trait IServiceManager {
    fn new() -> Self;
    fn register_service(&mut self, service_name: String, service: Box<dyn Service>);
}

pub trait ServiceManagerExt: Send + Sync  {
    fn try_handle(&self, path: String, request_post_body: RequestPostBody);
}


#[derive(Clone)]
pub struct ServiceManagerState {
    pub service_manager: Arc<dyn ServiceManagerExt>
}


pub struct ServiceManager {
    pub services: Arc<Mutex<HashMap<String, Arc<Mutex<Box<dyn Service>>>>>>,
}

// implementation for the ServiceManagerExt trait which ensures the ServiceManager implements
// the try_handle functionality
// TODO As soon as MessageQueue implementation is ready implement the communication
impl ServiceManagerExt for ServiceManager {
    fn try_handle(&self, path: String, post_body: RequestPostBody) {
        let binding = self.services.lock().unwrap();
        let service_option = &binding.get(&path);

        match service_option {
            Some(service) => {
                service.lock().unwrap().handle_request(post_body)
            }
            None => println!("no service found with name: {}", path)
        }
    }
}

impl IServiceManager for ServiceManager {

    // instantiation of ServiceManager instance.
    fn new() -> ServiceManager {
        let contents = file_helper::read_settings("config.setting");

        let mut my_manager = ServiceManager {
            services: Arc::new(Mutex::new(Default::default()))
        };
        match contents {
            Ok(content) => {
                // be aware to always end the config.setting file with an empty newline.
                // this ensures the correct functionality of the following code.
                if content.contains("\n") {
                    let lines = content.split('\n').collect::<Vec<&str>>();
                    for (_, line) in lines.iter().enumerate(){
                        info!("adding service {}", line);
                        // try find Service implementation in Lookup client
                        // and then register it in the manager
                        let client_option = lookup_client::find_service(line);
                        match client_option {
                            Some(client) => {
                                my_manager.register_service(line.to_string(), client);
                            },
                            None => error!("Unknown type '{}' in factory.", line)
                        }
                    }
                }
            },
            Err(e) => warn!("Could not read anything from the settings file. {:?}", e)
        }
        return my_manager
    }


    // registers the service in the Manager.
    fn register_service(&mut self, service_name: String, service: Box<dyn Service>) {
        println!("Adding service with name: '{}'", service_name);
        self.services.lock().unwrap().entry(service_name).or_insert(Arc::new(Mutex::new(service)));
    }
}