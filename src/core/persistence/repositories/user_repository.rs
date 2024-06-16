use sqlx::query;
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::contracts::dtos::user::FilteredUser;
use crate::core::persistence::core::persistence_errors::PersistenceError;
use crate::core::persistence::core::query_builder::{QueryBuilder, QueryClause};
use crate::core::persistence::core::query_builder::Sorting::Default;
use crate::core::persistence::core::table_name_supplier::TableNameSupplier;
use crate::core::persistence::table_names::TableName;
use crate::name_of;

pub async fn get_by_email(context: &ExecutionContext, email: &str) -> Result<FilteredUser,  GeneralServerError> {
    let mut whereclause: Vec<QueryClause> = vec![];
    whereclause.push(QueryClause::Equals(name_of!(email in FilteredUser)));


    let search_query = QueryBuilder::Select(Box::new(TableName::Users), Some(whereclause), Default, None);


    let row = query(&search_query.build_query())
        .bind(email)
        .fetch_optional(&*context.db.get_pool()).await?;
    let user_option: Option<FilteredUser> = match row {
        Some(user) => Some(user.into()),
        None => None
    };
    return if user_option.is_some(){
        Ok(user_option.unwrap())
    } else {
        let error = PersistenceError::CouldntFindSingle(TableName::Users.extract_table_name());
        Err(GeneralServerError{ message: error.get_err_message()})
    }
}
