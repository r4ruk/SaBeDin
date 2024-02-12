use serde::{Deserialize, Serialize};

// The request post body representation which can be further sent to processing Services
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPostBody {
    pub object: String,
    pub method: String,
    pub params: Vec<String>
}

#[derive(Serialize, Deserialize, Debug )]
pub struct ResponseBody {
    pub body: String,
}