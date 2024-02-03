use crate::core::contracts::basic_informations::RequestPostBody;
use crate::core::contracts::services::Service;

pub struct Client {
    name:  String
}

impl <'a> Service for Client {
    fn handle_request(&self, body: RequestPostBody) {
        // TODO add real data and functionality
        println!("handling request in clientservice {:?}", body)
    }

    fn instantiate() -> Box<dyn Service> {
        let c = Client { name: "ClientService".to_string()};
        return Box::new(c);
    }
}