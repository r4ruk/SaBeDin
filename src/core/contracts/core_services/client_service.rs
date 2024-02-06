use serde::{Deserialize, Serialize};
use serde_json::from_str;
use crate::core::contracts::basic_informations::RequestPostBody;
use crate::core::contracts::services::Service;

#[derive(Serialize, Deserialize)]
struct Client {
    name: String,
    age: usize,
    phones: Vec<String>
}

pub struct ClientService {
    name:  String
}

impl Service for ClientService {
    fn handle_request(&self, body: RequestPostBody) {
        // TODO add real data and functionality
        println!("handling request in clientservice {:?}", body);
        let obj:Client = from_str(&body.object).expect("cant parse body object");

        println!("name is {} and age is {}", obj.name, obj.age)
    }

    fn instantiate() -> Box<dyn Service> {
        let c = ClientService { name: "ClientService".to_string()};
        return Box::new(c);
    }
}