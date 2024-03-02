

// Test query builder here for right functionality as expected...

// [] variable input
// {} optional case

#[cfg(test)]
mod service_manager_test {
    use std::ptr::eq;
    use crate::core::persistence::query_builder::{QueryBuilder, QueryClause, SelectAmount};
    use crate::core::persistence::table_names::TableName;

    #[test]
    fn test_select_statements() {
        // SELECT [Amount] FROM [TABLENAME] {[WHERE CLAUSE]}
        let query = QueryBuilder::Select(SelectAmount::One, TableName::Users, None);
        let query_string = "SELECT 1 FROM users;";
        assert_eq!(query_string, query.build_query());

        // select with 1 where
        let mut where_clause: Vec<QueryClause> = vec![];
        where_clause.push(QueryClause::Equals("name".to_string()));

        let query = QueryBuilder::Select(SelectAmount::One, TableName::Users, Some(where_clause));
        let query_string = "SELECT 1 FROM users WHERE name = $1;";
        assert_eq!(query_string, query.build_query());


        // select with 2 where
        let mut where_clause: Vec<QueryClause> = vec![];
        where_clause.push(QueryClause::Equals("name".to_string()));
        where_clause.push(QueryClause::Equals("email".to_string()));

        let query = QueryBuilder::Select(SelectAmount::One, TableName::Users, Some(where_clause));
        let query_string = "SELECT 1 FROM users WHERE name = $1 AND email = $2;";
        assert_eq!(query_string, query.build_query());
    }

    fn test_insert_statements() {
        // INSERT INTO [TABLENAME]([fieldNames: Vec<String>]) VALUES ([fieldValues: Vec<(type: String, value: String)>])

    }

    fn test_delete_statements() {
        // DELETE FROM table_name WHERE condition;
    }

    fn test_update_statements() {
        // UPDATE table_name SET column1 = value1, column2 = value2 WHERE condition;
    }

}
