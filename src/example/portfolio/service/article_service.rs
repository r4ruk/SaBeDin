use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::contracts::base::query_params::QueryOptions;
use crate::example::portfolio::persistence::article_persistence;
use super::super::contracts::article::Article;

pub(crate)  async fn get_article_by_pkn(context: &ExecutionContext, pkn: &String) -> Result<Article, GeneralServerError> {
    return Ok(article_persistence::get_by_pkn(context, pkn).await?)
}

pub(crate) async fn get_all(context: &ExecutionContext, query_options: Option<QueryOptions>) -> Result<Vec<Article>, GeneralServerError> {

    let query_infos = match query_options {
        Some(q) => {q}
        None => {
            QueryOptions::default()
        }
    };

    return Ok(article_persistence::get_all(context, query_infos).await?)
}