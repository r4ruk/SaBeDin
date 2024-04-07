use crate::core::contracts::dependency_container::ExecutionContext;
use crate::core::contracts::errors::GeneralServerError;
use crate::example::portfolio::persistence::article_persistence;
use super::super::contracts::article::Article;

pub(crate) fn get_article_by_pkn(context: &ExecutionContext, pkn: &String) -> Result<Article, GeneralServerError> {
    return article_persistence::get_by_pkn(context, pkn)
}