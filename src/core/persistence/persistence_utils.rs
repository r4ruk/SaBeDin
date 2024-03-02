use sqlx::Error;
use crate::core::persistence::table_names::{TABLE_NAMES, TableName};

pub fn map_to_error_response(e: Error) -> String {
    let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Database error: {}", e),
            });

    return error_response.to_string()
}

pub fn extract_table_name(table_name: &TableName) -> String  {
    return if let Some(value) = TABLE_NAMES.get(table_name) {
        value.clone()
    } else {
        "".to_string()
    }
}