use std::collections::HashMap;
use crate::core::contracts::services::ClientHandler;
use std::sync::{Arc, Mutex};
use log::{error, info, warn};
use crate::core::contracts::basic_informations::{RequestPostBody, ResponseBody};
use crate::core::contracts::file_helper;
use crate::core::utils::utils;
use crate::service_manager::service_client_factory;


pub trait IServiceManager {
    fn new() -> Self;
    fn register_service(&mut self, service_name: String, service: Box<dyn ClientHandler>);
}

pub trait ServiceManagerExt: Send + Sync  {
    fn try_handle(&self, path: String, request_post_body: RequestPostBody);
    fn try_handle_query(&self, service: String, params: HashMap<String, String>) -> ResponseBody;
}


pub struct ServiceManager {
    pub services: Arc<Mutex<HashMap<String, Arc<Mutex<Box<dyn ClientHandler>>>>>>,
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
                service.lock().unwrap().handle_command(post_body)
            }
            None => println!("no service found with name: {}", path)
        }
    }

    fn try_handle_query(&self, service: String, params: HashMap<String, String>) -> ResponseBody {
        let binding = self.services.lock().unwrap();
        let service_option = &binding.get(&service);
        let mut response = ResponseBody{ body: "".to_string() };
        match service_option {
            Some(service) => {
                response = service.lock().unwrap().handle_query(params)
            }
            None => println!("no service found with name: {:?}", service)
        }
        return response
    }
}

impl IServiceManager for ServiceManager {

    // instantiation of ServiceManager instance.
    fn new() -> ServiceManager {
        let contents = file_helper::read_settings("config.setting");
        let os_specific_newline = utils::get_os_newline();

        let mut my_manager = ServiceManager {
            services: Arc::new(Mutex::new(Default::default()))
        };
        match contents {
            Ok(content) => {
                // be aware to always end the config.setting file with an empty newline.
                // this ensures the correct functionality of the following code.
                if content.contains(&os_specific_newline.clone()) {
                    let lines = content.split(&os_specific_newline).collect::<Vec<&str>>();
                    for (_, line) in lines.iter().enumerate() {
                        if line != "" {
                            info!("adding service {}", line);
                            // find Service implementation in service client factory
                            // and then register it in the manager
                            let client_option = service_client_factory::find_service(line);
                            match client_option {
                                Some(client) => {
                                    my_manager.register_service(line.to_string(), client);
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
    fn register_service(&mut self, service_name: String, service: Box<dyn ClientHandler>) {
        println!("Adding service with name: '{}'", service_name);
        self.services.lock().unwrap().entry(service_name).or_insert(Arc::new(Mutex::new(service)));
    }
}