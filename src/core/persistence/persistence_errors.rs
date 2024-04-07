
pub enum PersistenceError {
    CouldntFindSingle(String),
    CouldntFindAny(String),
    TableNotSupported(String)
}

impl PersistenceError {
    pub fn get_err_message(&self) -> String {
        return match self {
            PersistenceError::CouldntFindSingle(table_name) => {
                let message = format!("couldnt find single in {}", table_name);
                build_json_message(message)
            },

            PersistenceError::CouldntFindAny(table_name) => {
                let message = format!("couldnt find Any in {}", table_name);
                build_json_message(message)
            },
            PersistenceError::TableNotSupported(table_name) => {
                let message = format!("Table not supported {}", table_name);
                build_json_message(message)
            }
        }
    }
}

/// Function used to build single and only Json Response for persistence errors
fn build_json_message(msg: String) -> String {
    let error_response = serde_json::json!({
                    "status": "fail",
                    "message": msg,
                });
    return error_response.to_string()
}