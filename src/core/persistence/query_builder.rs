use std::any::Any;
use std::collections::HashMap;
use axum::extract::Query;
use crate::core::persistence::persistence_utils::extract_table_name;
use crate::core::persistence::table_names::{TABLE_NAMES, TableName};

pub enum SelectAmount {
    One,
    All,
    Amount(usize)
}

impl SelectAmount {
    fn get(&self) -> String {
        return match self {
            SelectAmount::One => "1".to_string(),
            SelectAmount::All => "8".to_string(),
            SelectAmount::Amount(amount) => amount.to_string()
        }
    }
}

// TODO sooner or later think about that so that queries can be made with AND and OR chaining
// pub enum ChainingStrategy {
//     AND,
//     OR
// }

pub enum QueryType {
    Select(SelectAmount, TableName, Option<Vec<QueryClause>>), // Amount, FromTableName, Optional Where Clause
    Insert(TableName, Vec<String>, Vec<String>),
    Update(TableName, Vec<(String,String)>, Option<Vec<QueryClause>>),
    Delete(TableName, Option<Vec<QueryClause>>)
}

/// Query clause taking propertyname and value(as string) as parameters.
pub enum QueryClause {
    Equals(String, String),
    Like(String, String),
    BiggerThan(String, String),
    SmallerThan(String, String)
}
impl QueryClause {
    fn get(&self) -> String {
        return match self {
            QueryClause::Equals(prop, val) => format!("{} = {}", prop, val),
            QueryClause::Like(prop, val) => format!("{} LIKE %{}%", prop, val),
            QueryClause::BiggerThan(prop, val) => format!("{} > {}", prop, val),
            QueryClause::SmallerThan(prop, val) => format!("{} < {}", prop, val),
        }
    }
}

// [] variable input
// {} optional case

// SELECT [Amount] FROM [TABLENAME] {[WHERE CLAUSE]}
// INSERT INTO [TABLENAME]([fieldNames: Vec<String>]) VALUES ([fieldValues: Vec<(type: String, value: String)>])

impl QueryType {
    fn build_query(&self) -> String {
        return match self {
            QueryType::Select(amount, table_name, where_clauses) => build_select_statement(amount, table_name, where_clauses),
            QueryType::Insert(table_name, field_names, field_values) => build_insert_statement(table_name, field_names, field_values),
            QueryType::Update(table_name, field_value_pairs, where_clauses) => build_update_statement(table_name, field_value_pairs, where_clauses),
            QueryType::Delete(table_name, where_clauses) => build_delete_statement(table_name, where_clauses),
        }
    }
}


fn build_select_statement(select_amount: &SelectAmount,
                          table_name: &TableName,
                          where_clauses: &Option<Vec<QueryClause>>)
    -> String {
    let binding = Vec::default();

    let wheres = match where_clauses {
        Some(w) => w,
        None => &binding
    };

    let mut query = format!("SELECT {} FROM {}", select_amount.get(), extract_table_name(table_name));
    let where_count = wheres.iter().count();
    if where_count > 0 {
        for (index, query_clause) in wheres.iter().enumerate() {
            if index < where_count - 1 {
                query = query + &query_clause.get() + " AND "
            } else {
                query = query + &query_clause.get() + ";"
            }
        }
    } else {
        query = query + ";";
    }
    return "".to_string()
}

fn build_insert_statement(table_name: &TableName, field_names: &Vec<String>, field_values: &Vec<String>) -> String {

    return "".to_string()
}

fn build_update_statement(table_name: &TableName,
                          field_value_pairs: &Vec<(String, String)>,
                          where_clauses: &Option<Vec<QueryClause>>)
    -> String {
    return "".to_string()
}


fn build_delete_statement(table_name: &TableName, where_clauses: &Option<Vec<QueryClause>>) -> String {
    return "".to_string()
}