use crate::service_manager::service_manager::IServiceManager;

pub trait IRequestHandler {
    fn new(service_manager: &impl IServiceManager) -> Self;
    fn handle_body(&self);
    fn fallthrough(&self) {
        // TODO add Publishing into event system to matching topic standard implementation
    }
}

