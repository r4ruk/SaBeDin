use sqlx::Error;

pub fn map_to_error_response(e: Error) -> String {
    let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Database error: {}", e),
            });

    return error_response.to_string()
}