

// Test query builder here for right functionality as expected...

// [] variable input
// {} optional case

#[cfg(test)]
mod query_builder_tests {
    use sqlx::query;
    use crate::core::persistence::query_builder::{QueryBuilder, QueryClause, SelectAmount};
    use crate::core::persistence::table_names::TableName;

    #[test]
    fn test_select_single_statements() {
        let query = QueryBuilder::Select(SelectAmount::One, TableName::Users, None);
        let query_string = "SELECT 1 FROM users";
        assert_eq!(query_string, query.build_query());
    }

    #[test]
    fn test_select_where_statements() {
        // select with 1 where
        let mut where_clause: Vec<QueryClause> = vec![];
        where_clause.push(QueryClause::Equals("name".to_string()));

        let query = QueryBuilder::Select(SelectAmount::One, TableName::Users, Some(where_clause));
        let query_string = "SELECT 1 FROM users WHERE name = $1";
        assert_eq!(query_string, query.build_query());


        // select with 2 where
        let mut where_clause: Vec<QueryClause> = vec![];
        where_clause.push(QueryClause::Equals("name".to_string()));
        where_clause.push(QueryClause::Equals("email".to_string()));

        let query = QueryBuilder::Select(SelectAmount::One, TableName::Users, Some(where_clause));
        let query_string = "SELECT 1 FROM users WHERE name = $1 AND email = $2";
        assert_eq!(query_string, query.build_query());
    }

    #[test]
    fn test_select_queryclause_statements() {
        let mut where_clause: Vec<QueryClause> = vec![];
        where_clause.push(QueryClause::BiggerThan("number".to_string()));

        let query = QueryBuilder::Select(SelectAmount::One, TableName::Users, Some(where_clause));
        let query_string = "SELECT 1 FROM users WHERE number > $1";
        assert_eq!(query_string, query.build_query());

        where_clause = vec![];
        where_clause.push(QueryClause::SmallerThan("number".to_string()));
        let query = QueryBuilder::Select(SelectAmount::One, TableName::Users, Some(where_clause));
        let query_string = "SELECT 1 FROM users WHERE number < $1";
        assert_eq!(query_string, query.build_query());


        where_clause = vec![];
        where_clause.push(QueryClause::StartsWith("name".to_string()));
        let query = QueryBuilder::Select(SelectAmount::One, TableName::Users, Some(where_clause));
        let query_string = "SELECT 1 FROM users WHERE name LIKE $1%";
        assert_eq!(query_string, query.build_query());

        where_clause = vec![];
        where_clause.push(QueryClause::EndsWith("name".to_string()));
        let query = QueryBuilder::Select(SelectAmount::One, TableName::Users, Some(where_clause));
        let query_string = "SELECT 1 FROM users WHERE name LIKE %$1";
        assert_eq!(query_string, query.build_query());

        where_clause = vec![];
        where_clause.push(QueryClause::Contains("name".to_string()));
        let query = QueryBuilder::Select(SelectAmount::One, TableName::Users, Some(where_clause));
        let query_string = "SELECT 1 FROM users WHERE name LIKE %$1%";
        assert_eq!(query_string, query.build_query());

        where_clause = vec![];
        where_clause.push(QueryClause::SmallerThan("number".to_string()));
        where_clause.push(QueryClause::Equals("name".to_string()));

        let query = QueryBuilder::Select(SelectAmount::One, TableName::Users, Some(where_clause));
        let query_string = "SELECT 1 FROM users WHERE number < $1 AND name = $2";
        assert_eq!(query_string, query.build_query());
    }

    #[test]
    fn test_select_all_statements() {
        let query = QueryBuilder::Select(SelectAmount::All, TableName::Users, None);
        let query_string = "SELECT * FROM users";
        assert_eq!(query_string, query.build_query());
    }

    #[test]
    fn test_select_subset_statements() {
        let query = QueryBuilder::Select(SelectAmount::Amount(5), TableName::Users, None);
        let query_string = "SELECT * FROM users LIMIT 5";
        assert_eq!(query_string, query.build_query());
    }

    #[test]
    fn test_insert_multiple_statements() {
        let query = QueryBuilder::Insert(TableName::Users,
                                         vec!["name".to_string(), "email".to_string(), "password".to_string()]);
        let expect_query = "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)";
        assert_eq!(expect_query, query.build_query());
    }
    #[test]
    fn test_insert_single_statements() {
        let query = QueryBuilder::Insert(TableName::Users,
                                         vec!["name".to_string()]);
        let expect_query = "INSERT INTO users (name) VALUES ($1)";
        assert_eq!(expect_query, query.build_query());
    }

    #[test]
    fn test_delete_whereincl_statements() {
        let mut where_clause: Vec<QueryClause> = vec![];
        where_clause.push(QueryClause::Equals("name".to_string()));
        where_clause.push(QueryClause::Equals("email".to_string()));

        let query = QueryBuilder::Delete(TableName::Users, Some(where_clause));
        let expect_query = "DELETE FROM users WHERE name = $1 AND email = $2";
        assert_eq!(expect_query, query.build_query());
    }
    #[test]
    fn test_delete_nowhere_statements() {
        let query = QueryBuilder::Delete(TableName::Users, None);
        let expect_query = "DELETE FROM users";
        assert_eq!(expect_query, query.build_query());
    }

    #[test]
    fn test_update_singleval_statements() {

        let mut where_clause: Vec<QueryClause> = vec![];
        where_clause.push(QueryClause::Equals("name".to_string()));
        where_clause.push(QueryClause::Equals("email".to_string()));

        let props = vec!["name".to_string()];

        let query = QueryBuilder::Update(TableName::Users, props, Some(where_clause));
        let query_expect = "UPDATE users SET name = $1 WHERE name = $2 AND email = $3";
        assert_eq!(query_expect, query.build_query());
    }
    #[test]
    fn test_update_multival_statements() {

        let mut where_clause: Vec<QueryClause> = vec![];
        where_clause.push(QueryClause::Equals("name".to_string()));
        where_clause.push(QueryClause::Equals("email".to_string()));

        let props = vec!["name".to_string(), "email".to_string()];

        let query = QueryBuilder::Update(TableName::Users, props, Some(where_clause));
        let query_expect = "UPDATE users SET name = $1, email = $2 WHERE name = $3 AND email = $4";
        assert_eq!(query_expect, query.build_query());
    }

}
