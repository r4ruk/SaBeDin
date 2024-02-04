use crate::core::contracts::basic_informations::RequestPostBody;


pub trait Service: 'static {
    fn handle_request(&self, body: RequestPostBody);
    fn instantiate() -> Box<dyn Service> where Self: Sized;
}


