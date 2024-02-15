use std::collections::HashMap;
use axum::response::Response;
use crate::core::contracts::basic_informations::{RequestPostBody, ResponseBody};

// definition for a Service, which can be used in the ServiceManager to route requests to.
pub trait Service: 'static + Send + Sync {
    fn handle_command(&self, body: RequestPostBody);
    fn handle_query(&self, params: HashMap<String, String>) -> ResponseBody;
    fn instantiate() -> Box<dyn Service> where Self: Sized;
}
