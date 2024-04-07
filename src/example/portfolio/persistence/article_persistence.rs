use serde_json::json;
use sqlx::query;
use crate::core::contracts::dependency_container::ExecutionContext;
use crate::core::contracts::errors::GeneralServerError;
use crate::example::portfolio::contracts::article::Article;
use crate::core::persistence::query_builder::{QueryBuilder, QueryClause, SelectAmount};
use crate::core::persistence::table_names::{TableName, TableNameSupplier};
use crate::example::portfolio::persistence::table_names::TableNamePortfolio;
use crate::name_of;

pub async fn get_by_pkn(context: &ExecutionContext, pkn: &str) -> Result<Article,  GeneralServerError> {
    let mut whereclause: Vec<QueryClause> = vec![];
    whereclause.push(QueryClause::Equals(name_of!(programming_key_name in Article)));
    let article_name_supplier: Box<dyn TableNameSupplier> = Box::new(TableNamePortfolio::Article);

    let search_query = QueryBuilder::Select(SelectAmount::One, article_name_supplier, Some(whereclause));


    let row = query(&search_query.build_query())
                .bind(pkn)
        .fetch_optional(&context.db).await?;
    let article_option = match row {
        Some(article) => Some(article.into()),
        None => None
    };
    return if article_option.is_some(){
        Ok(article_option.unwrap())
    } else {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "couldnt find article",
        });
        let err = GeneralServerError{ message: error_response.to_string()};
        Err(err)
    }
}