use std::collections::HashMap;
use crate::core::contracts::services::Service;
use std::{fs, env};
use std::option::Option;
use log::{error, info, warn};
use crate::service_manager::lookup_client;


pub trait IServiceManager {
    fn new() -> Self;
    fn get(&self, service_name: String) -> Option<&Box<dyn Service>>;
    fn register_service(&mut self, service_name: String, service: Box<dyn Service>);
}


pub struct ServiceManager {
    pub services: HashMap<String,  Box<dyn Service>>,
}

impl IServiceManager for ServiceManager {
    fn new() -> ServiceManager {
        let current_dir = env::current_dir().expect("should be able to open directory");
        let path= current_dir.join("config.setting");
        let contents = fs::read_to_string(path);

        let mut my_manager = ServiceManager {
            services: Default::default(),
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

    fn get(&self, service_name: String) -> Option<&Box<dyn Service>> {
        if let Some(value) = self.services.get(&service_name) {
            return Some(value)
        } else {
            warn!("did not find service with the name: '{}'", service_name);
            None
        }
    }

    fn register_service(&mut self, service_name: String, service: Box<dyn Service>) {
        self.services
            .entry(service_name)
            .or_insert(service);
    }
}