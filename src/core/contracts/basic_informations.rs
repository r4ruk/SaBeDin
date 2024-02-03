use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPostBody {
    pub method: String,
    pub params: Vec<String>
}