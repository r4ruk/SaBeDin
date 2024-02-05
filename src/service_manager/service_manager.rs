use std::collections::HashMap;
use crate::core::contracts::services::Service;
use std::{fs, env};
use std::sync::{Arc, Mutex};
use log::{error, info, warn};
use crate::core::contracts::basic_informations::RequestPostBody;
use crate::service_manager::lookup_client;


pub trait IServiceManager {
    fn new() -> Self;
    fn register_service(&mut self, service_name: String, service: Box<dyn Service>);
}

pub trait ServiceManagerExt: Send + Sync  {
    fn try_handle(&self, path: String);
}


#[derive(Clone)]
pub struct ServiceManagerState {
    pub service_manager: Arc<dyn ServiceManagerExt>
}


pub struct ServiceManager {
    pub services: Arc<Mutex<HashMap<String, Arc<Mutex<Box<dyn Service>>>>>>,
}

impl ServiceManagerExt for ServiceManager {
    fn try_handle(&self, path: String) {
        let binding = self.services.lock().unwrap();
        let service = &binding.get(&path);

        match service {
            Some(T) => {
                T.lock().unwrap().handle_request(RequestPostBody{method: "test".to_string(), params: vec!["1".to_string()] })
            }
            None => println!("no service found with name: {}", path)
        }
    }
}

impl IServiceManager for ServiceManager {
    fn new() -> ServiceManager {
        let current_dir = env::current_dir().expect("should be able to open directory");
        let path= current_dir.join("config.setting");
        let contents = fs::read_to_string(path);

        let mut my_manager = ServiceManager {
            services: Arc::new(Mutex::new(Default::default()))
        };
        match contents {
            Ok(content) => {
                if content.contains("\n") {
                    let lines = content.split('\n').collect::<Vec<&str>>();
                    for (_, line) in lines.iter().enumerate(){
                        info!("adding service {}", line);
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


    fn register_service(&mut self, service_name: String, service: Box<dyn Service>) {
        // self.services
        //     .entry(service_name)
        //     .or_insert(service);
        println!("Adding service with name: '{}'", service_name);
        self.services.lock().unwrap().entry(service_name).or_insert(Arc::new(Mutex::new(service)));
    }
}