use sqlx::query;
use crate::core::contracts::base::dependency_container::ExecutionContext;
use crate::core::contracts::base::errors::GeneralServerError;
use crate::core::contracts::base::query_params::QueryOptions;
use crate::core::persistence::core::persistence_errors::PersistenceError;
use crate::example::portfolio::contracts::article::Article;
use crate::core::persistence::core::query_builder::{QueryBuilder, QueryClause};
use crate::core::persistence::core::query_builder::Sorting::Default;
use crate::core::persistence::core::table_name_supplier::TableNameSupplier;
use crate::example::portfolio::persistence::table_names::TableNamePortfolio;
use crate::name_of;

pub async fn get_by_pkn(context: &ExecutionContext, pkn: &str) -> Result<Article,  GeneralServerError> {
    let mut whereclause: Vec<QueryClause> = vec![];
    whereclause.push(QueryClause::Equals(name_of!(programming_key_name in Article)));
    let article_name_supplier: Box<dyn TableNameSupplier> = Box::new(TableNamePortfolio::Article);

    let search_query = QueryBuilder::Select(article_name_supplier, Some(whereclause), Default, None);


    let row = query(&search_query.build_query())
                .bind(pkn)
        .fetch_optional(&*context.db.get_pool()).await?;
    let article_option = match row {
        Some(article) => Some(article.into()),
        None => None
    };
    return if article_option.is_some(){
        Ok(article_option.unwrap())
    } else {
        let error = PersistenceError::CouldntFindSingle(TableNamePortfolio::Article.extract_table_name());
        Err(GeneralServerError{ message: error.get_err_message()})
    }
}

/// Function returns all articles stored
pub async fn get_all(context: &ExecutionContext, query_options: QueryOptions) -> Result<Vec<Article>, GeneralServerError> {

    let search_query =
        QueryBuilder::Select(
            Box::new(TableNamePortfolio::Article),
            Some(query_options.queries),
            query_options.sorting_information,
            Some(query_options.paging_information));

    let rows = query(&search_query.build_query())
        .fetch_all(&*context.db.get_pool()).await?;
    let mut all_articles = Vec::new();
    for row in rows {
        all_articles.push(row.into())
    };
    return if all_articles.len() > 0 {
        Ok(all_articles)
    } else {
        let error = PersistenceError::CouldntFindSingle(TableNamePortfolio::Article.extract_table_name());
        Err(GeneralServerError{ message: error.get_err_message()})
    }
}