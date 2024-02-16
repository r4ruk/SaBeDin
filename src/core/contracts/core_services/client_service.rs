use std::collections::HashMap;
use std::hash::Hash;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
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

    // sample
    // TODO introduce BL layer which is giving functions which actually retrieve information
    fn handle_query(&self, params: HashMap<String, String>) -> ResponseBody {
        println!("{:?}", params);
        if params.len() == 1 {
            let (key, val) = params.iter().nth(0).unwrap();
            let mut ret_val: Option<Client> = None;
            match key.as_str() {
                "id" => ret_val = Some(helper_id(val.as_str())),
                "name" => ret_val = Some(helper_name(val.as_str())),
                _ => {
                    println!("wrong param given");
                }
            }
            if let Some(ret) = ret_val {
                return ResponseBody{
                    body: json!(ret).to_string()
                }
            }
        }
        return ResponseBody {
            body: "hello from handle_query".to_string(),
        }
    }

    // method create a boxed instance of the actual Service
    fn instantiate() -> Box<dyn Service> {
        let c = ClientService { name: "ClientService".to_string()};
        return Box::new(c);
    }
}
// TODO remove following lines as soon as BL layer is up
fn helper_id(id: &str) -> Client{
    return Client{ name: "Hans".to_string(), age: 1, phones: vec![] }
}
fn helper_name(name: &str) -> Client{
    return Client{ name: "NameParamClient".to_string(), age: 12, phones: vec![] }
}
