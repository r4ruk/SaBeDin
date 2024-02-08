use crate::core::contracts::basic_informations::RequestPostBody;

// definition for a Service, which can be used in the ServiceManager to route requests to.
pub trait Service: 'static + Send + Sync {
    fn handle_request(&self, body: RequestPostBody);
    fn instantiate() -> Box<dyn Service> where Self: Sized;
}
