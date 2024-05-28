use uuid::Uuid;
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::persistence::core::persistence_utils;
use crate::core::persistence::core::query_builder::{QueryBuilder, QueryClause};
use crate::core::persistence::core::query_builder::Sorting::Default;
use crate::core::persistence::table_names::TableName;
use core::option::Option;
use crate::core::contracts::dtos::idempotency_info::IdempotencyObject;
use crate::name_of;


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

pub async fn create_idempotency_key(transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>, idempotency_object: IdempotencyObject) -> Result<(), GeneralServerError> {
    let query_builder = QueryBuilder::Insert(Box::new(TableName::Idempotency), map_object_props_to_vec());

    sqlx::query(&query_builder.build_query())
            .bind(idempotency_object.user_id)
            .bind(idempotency_object.idempotency_key)
            .bind(idempotency_object.response_status_code)
            .bind(idempotency_object.response_body)
            .execute(&mut **transaction)
            .await
            .map_err(|e| {
                GeneralServerError { message: persistence_utils::map_to_error_response(e) }
            })?;

    Ok(())
}

pub async fn update_idempotency_key(transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>, idempotency_object: IdempotencyObject) -> Result<(), GeneralServerError> {
    let mut whereclauses: Vec<QueryClause> = vec!();
    whereclauses.push(QueryClause::Equals(idempotency_object.clone().user_id.to_string()));
    whereclauses.push(QueryClause::Equals(idempotency_object.clone().idempotency_key));

    let updatefields = vec![name_of!(response_status_code in IdempotencyObject), name_of!(response_body in IdempotencyObject)];

    let query_builder = QueryBuilder::Update(Box::new(TableName::Idempotency),updatefields, Some(whereclauses));

    sqlx::query(&query_builder.build_query())
            .bind(idempotency_object.response_status_code)
            .bind(idempotency_object.response_body)
            .bind(idempotency_object.user_id)
            .bind(idempotency_object.idempotency_key)
            .execute(&mut **transaction)
            .await
            .map_err(|e| {
                GeneralServerError { message: persistence_utils::map_to_error_response(e) }
            })?;

    Ok(())
}


fn map_object_props_to_vec() -> Vec<String>{
    vec![name_of!(user_id in IdempotencyObject),
         name_of!(idempotency_key in IdempotencyObject),
         name_of!(response_status_code in IdempotencyObject),
         name_of!(response_body in IdempotencyObject)]
}