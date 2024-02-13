use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use crate::core::contracts::basic_informations::{RequestPostBody, ResponseBody};
use crate::core::contracts::services::Service;

// template definition for further work on real entities and services.
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
    fn handle_command(&self, body: RequestPostBody) {
        // TODO add real data and functionality
        println!("handling request in clientservice {:?}", body);

        // parsing body's object which can be any 'serializable/deserializable' JSON representation.
        let client:Client = from_str(&body.object).expect("cant parse body object");

        // ensuring the deserialization worked
        println!("name is {} and age is {}", client.name, client.age)
    }

    fn handle_query(&self, params: String) -> ResponseBody{
        println!("{}", params);
        return ResponseBody {
            body: params.to_string(),
        }
    }

    // method create a boxed instance of the actual Service
    fn instantiate() -> Box<dyn Service> {
        let c = ClientService { name: "ClientService".to_string()};
        return Box::new(c);
    }
}