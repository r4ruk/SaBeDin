use std::fmt::format;
use std::ops::Deref;
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
            SelectAmount::All => "*".to_string(),
            SelectAmount::Amount(amount) => "*".to_string()
        }
    }

    fn get_additional(&self) -> String {
        return match self {
            SelectAmount::Amount(amount) => format!(" LIMIT {}", amount),
            _ => "".to_string()
        }
    }
}

// TODO sooner or later think about that so that queries can be made with AND and OR chaining
// pub enum ChainingStrategy {
//     AND,
//     OR
// }

pub enum QueryBuilder {
    Select(SelectAmount, TableName, Option<Vec<QueryClause>>), // Amount, FromTableName, Optional Where Clause
    Insert(TableName, Vec<String>), // TableToInsert, FieldNames
    Update(TableName, Vec<(String,String)>, Option<Vec<QueryClause>>),
    Delete(TableName, Option<Vec<QueryClause>>)
}

/// Query clause taking propertyname as parameter.
pub enum QueryClause {
    Equals(String),
    StartsWith(String),
    EndsWith(String),
    Contains(String),
    BiggerThan(String),
    SmallerThan(String)
}
impl QueryClause {
    fn get(&self, index: usize) -> String {
        return match self {
            QueryClause::Equals(prop) => format!("{} = ${}", prop, index),
            QueryClause::StartsWith(prop) => format!("{} LIKE ${}%", prop, index),
            QueryClause::EndsWith(prop) => format!("{} LIKE %${}", prop, index),
            QueryClause::Contains(prop) => format!("{} LIKE %${}%", prop, index),
            QueryClause::BiggerThan(prop) => format!("{} > ${}", prop, index),
            QueryClause::SmallerThan(prop) => format!("{} < ${}", prop, index),
        }
    }
}

// [] variable input
// {} optional case

// SELECT [Amount] FROM [TABLENAME] {[WHERE CLAUSE]}

impl QueryBuilder {
    pub(crate) fn build_query(&self) -> String {
        return match self {
            QueryBuilder::Select(amount, table_name, where_clauses) => build_select_statement(amount, table_name, where_clauses),
            QueryBuilder::Insert(table_name, field_names) => build_insert_statement(table_name, field_names),
            QueryBuilder::Update(table_name, field_value_pairs, where_clauses) => build_update_statement(table_name, field_value_pairs, where_clauses),
            QueryBuilder::Delete(table_name, where_clauses) => build_delete_statement(table_name, where_clauses),
        }
    }
}


fn build_select_statement(select_amount: &SelectAmount,
                          table_name: &TableName,
                          where_clauses: &Option<Vec<QueryClause>>)
    -> String {

    let mut query = format!("SELECT {} FROM {}", select_amount.get(), extract_table_name(table_name));
    query = query + &build_where_clause(where_clauses);
    query = query + &select_amount.get_additional() + ";";
    return query.to_string()
}

fn build_where_clause(where_clauses:  &Option<Vec<QueryClause>>) -> String{
    let binding = Vec::default();

    let wheres = match where_clauses {
        Some(w) => w,
        None => &binding
    };

    let where_count = wheres.iter().count();
    let mut query = "".to_string();
    if where_count > 0 {
        query = query + " WHERE ";
        for (index, query_clause) in wheres.iter().enumerate() {
            if index < where_count - 1 {
                query = query + &query_clause.get(index + 1) + " AND "
            } else {
                query = query + &query_clause.get(index + 1)
            }
        }
    }
    return query
}

// INSERT INTO [TABLENAME]([fieldNames: Vec<String>]) VALUES ([fieldValues: Vec<(type: String, value: String)>])
fn build_insert_statement(table_name: &TableName, field_names: &Vec<String>) -> String {
    let mut field_count = field_names.iter().count();
        // if no fields provided
        // return empty
    if field_count == 0{
        return "".to_string()
    }

    let mut query = format!("INSERT INTO {} ({}) VALUES ({})",
                            extract_table_name(table_name),
                            build_fields_chaining(field_names),
                            build_values_chaining(field_count));
    return query
}

fn build_values_chaining(field_count: usize) -> String {
    let mut val_chain = "".to_string();
    if field_count == 1 {
        return "$1".to_string()
    } else {
        for i in 0..field_count {
            val_chain = val_chain + &format!("${}", i+1);
            if i < field_count-1 {
                val_chain = val_chain  + ", ";
            }
        }
    }
    return val_chain
}

fn build_fields_chaining(values: &Vec<String>) -> String {
    let mut chain: String = "".to_string();
    let values_count = values.iter().count();
    if values_count > 1 {
        for (index, element) in values.iter().enumerate() {
            chain = chain + element;
            if index < values_count -1 {
                chain = chain + ", ";
            }
        }
    } else {
        return values.first().unwrap().to_string()
    }
    return chain
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