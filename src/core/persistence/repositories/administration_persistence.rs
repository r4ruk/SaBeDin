use uuid::Uuid;
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::persistence::core::persistence_utils;
use crate::core::persistence::core::query_builder::{QueryBuilder, QueryClause};
use crate::core::persistence::core::query_builder::Sorting::Default;
use crate::core::persistence::table_names::TableName;
use core::option::Option;


pub async fn idempotency_key_exists(context: &&ExecutionContext, key: String, user_id: Uuid) -> Result<Option<bool>, GeneralServerError> {
    let where_query = vec![
            QueryClause::Equals("idempotency_key".to_string()),
            QueryClause::Equals("user_id".to_string())];

    let select_exists = QueryBuilder::Select(Box::new(TableName::Idempotency), Some(where_query), Default, None);

    let key_exists: Option<bool> =
        sqlx::query_scalar(&format!("SELECT EXISTS({})", select_exists.build_query()))
            .bind(key.to_owned().to_ascii_lowercase())
            .bind(user_id.to_owned())
            .fetch_one(&*context.db.get_pool())
            .await
            .map_err(|e| {
                GeneralServerError { message: persistence_utils::map_to_error_response(e) }
            })?;

    Ok(key_exists)
}