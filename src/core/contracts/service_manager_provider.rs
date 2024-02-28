use std::collections::HashMap;
use crate::core::contracts::basic_informations::{RequestPostBody, ResponseBody};

pub trait ServiceManagerProvider: Send + Sync  {
    fn try_handle(&self, path: String, request_post_body: RequestPostBody);
    fn try_handle_query(&self, service: String, params: HashMap<String, String>) -> ResponseBody;
}