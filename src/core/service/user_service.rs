use crate::core::contracts::dtos::user::FilteredUser;
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::persistence::repositories::user_repository;

pub async fn get_user_by_email(context: &ExecutionContext, email: &str) -> Result<FilteredUser, GeneralServerError> {
    let result = user_repository::get_by_email(context, email).await?;
    return Ok(result.unwrap())
}