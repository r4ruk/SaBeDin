use sqlx::{query, query_as};
use crate::core::persistence::query_builder::QueryBuilder;
use crate::core::contracts::dependency_container::ExecutionContext;
use crate::core::contracts::errors::GeneralServerError;
use crate::core::contracts::user::{FilteredUser, LoginUserData, RegisterUserData, User};
use crate::core::persistence::persistence_utils;
use crate::core::persistence::query_builder::{QueryClause, SelectAmount};
use crate::core::persistence::table_names::TableName;
use crate::name_of;


pub async fn login_user(context: &ExecutionContext, user_data: LoginUserData) -> Result<FilteredUser, GeneralServerError> {
    let mut where_clause: Vec<QueryClause> = vec![];
    where_clause.push(QueryClause::Equals(name_of!(email in LoginUserData)));
    let search_query = QueryBuilder::Select(SelectAmount::All, TableName::Users, Some(where_clause));

    let row = query(&search_query.build_query())
                            .bind(user_data.email)
                            .fetch_optional(&context.db)
                            .await?;
    let user_option: Option<FilteredUser> = match row {
        Some(user) => Some(user.into()),
        None => None
    };

    return if user_option.is_some() {
        Ok(user_option.unwrap())
    } else {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid email or password",
        });
        let err = GeneralServerError { message: error_response.to_string() };
        Err(err)
    }
}

pub async fn register_user(context: &ExecutionContext, user_data: RegisterUserData) -> Result<FilteredUser, GeneralServerError>{
    let user_exists = check_user_exists(&context, &user_data).await?;

    match user_exists {
        Some(exists_state) => {
            if exists_state {
                return Err(GeneralServerError{message: "user already exists".to_string()})
            }
        }
        None => (),
    }

    let query_builder = QueryBuilder::Insert(TableName::Users, vec![name_of!(name in RegisterUserData),
                                                name_of!(email in RegisterUserData),
                                                name_of!(password in RegisterUserData)]);


    let res = sqlx::query(&query_builder.build_query())
        .bind(user_data.name)
        .bind(user_data.email)
        .bind(user_data.password)
        .fetch_one(&context.db)
        .await.map_err(|e| {
            GeneralServerError{message: persistence_utils::map_to_error_response(e)}
        })?;
    return Ok(res.into());
}

async fn check_user_exists(context: &&ExecutionContext, user_data: &RegisterUserData) -> Result<Option<bool>, GeneralServerError> {
    let where_query = vec![QueryClause::Equals(name_of!(email in RegisterUserData))];
    let select_exists = QueryBuilder::Select(SelectAmount::One, TableName::Users, Some(where_query));

    let user_exists: Option<bool> =
        sqlx::query_scalar(&format!("SELECT EXISTS({})", select_exists.build_query()))
            .bind(user_data.email.to_owned().to_ascii_lowercase())
            .fetch_one(&context.db)
            .await
            .map_err(|e| {
                GeneralServerError { message: persistence_utils::map_to_error_response(e) }
            })?;
    Ok(user_exists)
}



