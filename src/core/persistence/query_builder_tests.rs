

// Test query builder here for right functionality as expected...

// [] variable input
// {} optional case

#[cfg(test)]
mod query_builder_tests {
    use crate::core::contracts::user::User;
    use crate::core::persistence::query_builder::{PagingQuery, QueryBuilder, QueryClause, Sorting};
    use crate::core::persistence::query_builder::Sorting::{Ascending, Default, Descending};
    use crate::core::persistence::table_names::TableName;
    use crate::name_of;

    #[test]
    fn test_select_single_statements() {
        let query = QueryBuilder::Select(Box::new(TableName::Users), None, Default, Some(PagingQuery{ amount_of_items: 1, page_num: 0 }));
        let query_string = "SELECT * FROM users ORDER BY id LIMIT 1 OFFSET 0";
        assert_eq!(query_string, query.build_query());
    }

    #[test]
    fn test_select_where_statements() {
        // select with 1 where
        let mut where_clause: Vec<QueryClause> = vec![];
        where_clause.push(QueryClause::Equals("name".to_string()));

        let query = QueryBuilder::Select(Box::new(TableName::Users), Some(where_clause), Default, Some(PagingQuery{ amount_of_items: 1, page_num: 0}));
        let query_string = "SELECT * FROM users WHERE name = $1 ORDER BY id LIMIT 1 OFFSET 0";
        assert_eq!(query_string, query.build_query());


        // select with 2 where
        let mut where_clause: Vec<QueryClause> = vec![];
        where_clause.push(QueryClause::Equals("name".to_string()));
        where_clause.push(QueryClause::Equals("email".to_string()));

        let query = QueryBuilder::Select(Box::new(TableName::Users), Some(where_clause), Default, Some(PagingQuery { amount_of_items: 1, page_num: 0 }));
        let query_string = "SELECT * FROM users WHERE name = $1 AND email = $2 ORDER BY id LIMIT 1 OFFSET 0";
        assert_eq!(query_string, query.build_query());
    }

    #[test]
    fn test_select_queryclause_statements() {
        let mut where_clause: Vec<QueryClause> = vec![];
        where_clause.push(QueryClause::BiggerThan("number".to_string()));

        let query = QueryBuilder::Select(Box::new(TableName::Users), Some(where_clause), Default, Some(PagingQuery { amount_of_items: 1, page_num: 0 }));
        let query_string = "SELECT * FROM users WHERE number > $1 ORDER BY id LIMIT 1 OFFSET 0";
        assert_eq!(query_string, query.build_query());

        where_clause = vec![];
        where_clause.push(QueryClause::SmallerThan("number".to_string()));
        let query = QueryBuilder::Select(Box::new(TableName::Users), Some(where_clause), Default, Some(PagingQuery { amount_of_items: 1, page_num: 0 }));
        let query_string = "SELECT * FROM users WHERE number < $1 ORDER BY id LIMIT 1 OFFSET 0";
        assert_eq!(query_string, query.build_query());


        where_clause = vec![];
        where_clause.push(QueryClause::StartsWith("name".to_string()));
        let query = QueryBuilder::Select(Box::new(TableName::Users), Some(where_clause), Default, None);
        let query_string = "SELECT * FROM users WHERE name LIKE $1% ORDER BY id";
        assert_eq!(query_string, query.build_query());

        where_clause = vec![];
        where_clause.push(QueryClause::EndsWith("name".to_string()));
        let query = QueryBuilder::Select(Box::new(TableName::Users), Some(where_clause), Default, Some(PagingQuery { amount_of_items: 1, page_num: 0 }));
        let query_string = "SELECT * FROM users WHERE name LIKE %$1 ORDER BY id LIMIT 1 OFFSET 0";
        assert_eq!(query_string, query.build_query());

        where_clause = vec![];
        where_clause.push(QueryClause::Contains("name".to_string()));
        let query = QueryBuilder::Select(Box::new(TableName::Users), Some(where_clause), Default, Some(PagingQuery { amount_of_items: 1, page_num: 0 }));
        let query_string = "SELECT * FROM users WHERE name LIKE %$1% ORDER BY id LIMIT 1 OFFSET 0";
        assert_eq!(query_string, query.build_query());

        where_clause = vec![];
        where_clause.push(QueryClause::SmallerThan("number".to_string()));
        where_clause.push(QueryClause::Equals("name".to_string()));

        let query = QueryBuilder::Select(Box::new(TableName::Users), Some(where_clause), Default, Some(PagingQuery { amount_of_items: 1, page_num: 0 }));
        let query_string = "SELECT * FROM users WHERE number < $1 AND name = $2 ORDER BY id LIMIT 1 OFFSET 0";
        assert_eq!(query_string, query.build_query());
    }

    #[test]
    fn test_select_all_statements() {
        let query = QueryBuilder::Select(Box::new(TableName::Users), None, Default, None);
        let query_string = "SELECT * FROM users ORDER BY id";
        assert_eq!(query_string, query.build_query());
    }

    #[test]
    fn test_select_subset_statements() {
        let query = QueryBuilder::Select(Box::new(TableName::Users), None, Sorting::Default, Some(PagingQuery{ amount_of_items: 5, page_num: 0 }));
        let query_string = "SELECT * FROM users ORDER BY id LIMIT 5 OFFSET 0";
        assert_eq!(query_string, query.build_query());
    }

    #[test]
    fn test_insert_multiple_statements() {
        let query = QueryBuilder::Insert(Box::new(TableName::Users),
                                         vec!["name".to_string(), "email".to_string(), "password".to_string()]);
        let expect_query = "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)";
        assert_eq!(expect_query, query.build_query());
    }

    #[test]
    fn test_insert_single_statements() {
        let query = QueryBuilder::Insert(Box::new(TableName::Users),
                                         vec!["name".to_string()]);
        let expect_query = "INSERT INTO users (name) VALUES ($1)";
        assert_eq!(expect_query, query.build_query());
    }

    #[test]
    fn test_delete_whereincl_statements() {
        let mut where_clause: Vec<QueryClause> = vec![];
        where_clause.push(QueryClause::Equals("name".to_string()));
        where_clause.push(QueryClause::Equals("email".to_string()));

        let query = QueryBuilder::Delete(Box::new(TableName::Users), Some(where_clause));
        let expect_query = "DELETE FROM users WHERE name = $1 AND email = $2";
        assert_eq!(expect_query, query.build_query());
    }
    #[test]
    fn test_delete_nowhere_statements() {
        let query = QueryBuilder::Delete(Box::new(TableName::Users), None);
        let expect_query = "DELETE FROM users";
        assert_eq!(expect_query, query.build_query());
    }

    #[test]
    fn test_update_singleval_statements() {

        let mut where_clause: Vec<QueryClause> = vec![];
        where_clause.push(QueryClause::Equals("name".to_string()));
        where_clause.push(QueryClause::Equals("email".to_string()));

        let props = vec!["name".to_string()];

        let query = QueryBuilder::Update(Box::new(TableName::Users), props, Some(where_clause));
        let query_expect = "UPDATE users SET name = $1 WHERE name = $2 AND email = $3";
        assert_eq!(query_expect, query.build_query());
    }
    #[test]
    fn test_update_multival_statements() {

        let mut where_clause: Vec<QueryClause> = vec![];
        where_clause.push(QueryClause::Equals("name".to_string()));
        where_clause.push(QueryClause::Equals("email".to_string()));

        let props = vec!["name".to_string(), "email".to_string()];

        let query = QueryBuilder::Update(Box::new(TableName::Users), props, Some(where_clause));
        let query_expect = "UPDATE users SET name = $1, email = $2 WHERE name = $3 AND email = $4";
        assert_eq!(query_expect, query.build_query());
    }

    #[test]
    fn test_paging_basic_statements() {
        let mut where_clause: Vec<QueryClause> = vec![];
        where_clause.push(QueryClause::Equals("name".to_string()));
        where_clause.push(QueryClause::Equals("email".to_string()));

        let mut query = QueryBuilder::Select(Box::new(TableName::Users),
                                         Some(where_clause),
                                         Default,
                                         Some(PagingQuery{ amount_of_items: 1,
                                             page_num: 0 }));

        let mut query_expect = "SELECT * FROM users WHERE name = $1 AND email = $2 ORDER BY id LIMIT 1 OFFSET 0";
        assert_eq!(query_expect, query.build_query());

        query = QueryBuilder::Select(Box::new(TableName::Users),
                                         None,
                                         Default,
                                         Some(PagingQuery{ amount_of_items: 5,
                                             page_num: 0}));
        query_expect = "SELECT * FROM users ORDER BY id LIMIT 5 OFFSET 0";
        assert_eq!(query_expect, query.build_query());


        query = QueryBuilder::Select(Box::new(TableName::Users),
                                         None,
                                         Default,
                                         Some(PagingQuery{ amount_of_items: 5,
                                             page_num: 1}));
        query_expect = "SELECT * FROM users ORDER BY id LIMIT 5 OFFSET 5";
        assert_eq!(query_expect, query.build_query());

        query = QueryBuilder::Select(Box::new(TableName::Users),
                                         None,
                                         Default,
                                         Some(PagingQuery{ amount_of_items: 5,
                                             page_num: 2}));
        query_expect = "SELECT * FROM users ORDER BY id LIMIT 5 OFFSET 10";
        assert_eq!(query_expect, query.build_query());
    }

    #[test]
    fn test_sorting() {

        let mut query = QueryBuilder::Select(Box::new(TableName::Users),
                                             None,
                                             Ascending(vec![name_of!(email in User), name_of!(name in User)]),
                                             None);
        let mut query_expect = "SELECT * FROM users ORDER BY email,name";
        assert_eq!(query_expect, query.build_query());

        query = QueryBuilder::Select(Box::new(TableName::Users),
                                     None,
                                     Descending(vec![name_of!(email in User), name_of!(name in User)]),
                                     None);
        query_expect = "SELECT * FROM users ORDER BY email DESC,name DESC";
        assert_eq!(query_expect, query.build_query());
    }

    #[test]
    fn test_sorting_paging() {
        let mut where_clause: Vec<QueryClause> = vec![];
        where_clause.push(QueryClause::Equals("name".to_string()));
        where_clause.push(QueryClause::Equals("email".to_string()));

        let mut query = QueryBuilder::Select(Box::new(TableName::Users),
                                             Some(where_clause),
                                             Ascending(vec![name_of!(email in User), name_of!(name in User)]),
                                             Some(PagingQuery{ amount_of_items: 1, page_num: 0 }));

        let mut query_expect = "SELECT * FROM users WHERE name = $1 AND email = $2 ORDER BY email,name LIMIT 1 OFFSET 0";
        assert_eq!(query_expect, query.build_query());
    }
}
