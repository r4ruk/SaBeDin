use crate::core::contracts::dependency_container::ExecutionContext;
use crate::core::contracts::errors::GeneralServerError;
use super::super::contracts::article::Article;

pub(crate) fn get_article_by_pkn(context: &ExecutionContext, pkn: &String) -> Result<Article, GeneralServerError> {
    return Ok(Article {
        id: Default::default(),
        programming_key_name: pkn.to_string(),
        title: "".to_string(),
        contents: "".to_string(),
        tags: "".to_string(),
        created_at: Default::default(),
    })
}