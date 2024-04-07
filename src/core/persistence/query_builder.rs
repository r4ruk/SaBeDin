use crate::core::persistence::table_names::{TableName, TableNameSupplier};

#[allow(unused)]
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
            SelectAmount::Amount(_) => "*".to_string()
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

#[allow(unused)]
pub enum QueryBuilder {
    /// params: Amount, FromTableName, Option\<Vec\<QueryClause\>\>
    Select(SelectAmount, Box<dyn TableNameSupplier>, Option<Vec<QueryClause>>),

    /// params: TableName, Vec\<String\> which represents field names
    Insert(Box<dyn TableNameSupplier>, Vec<String>),

    /// params: TableName, Vec\<String\> which represents field names, Option\<Vec\<QueryClause\>\>
    Update(Box<dyn TableNameSupplier>, Vec<String>, Option<Vec<QueryClause>>),

    /// params: TableName and Option\<Vec\<QueryClause\>\>
    Delete(Box<dyn TableNameSupplier>, Option<Vec<QueryClause>>)
}

/// Query clause taking propertyname as parameter.
#[allow(unused)]
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

impl QueryBuilder {
    pub fn build_query(&self) -> String {
        return match self {
            QueryBuilder::Select(amount, table_name, where_clauses) => build_select_statement(amount, table_name, where_clauses),
            QueryBuilder::Insert(table_name, field_names) => build_insert_statement(table_name, field_names),
            QueryBuilder::Update(table_name, field_names, where_clauses) => build_update_statement(table_name, field_names, where_clauses),
            QueryBuilder::Delete(table_name, where_clauses) => build_delete_statement(table_name, where_clauses),
        }
    }
}

fn build_select_statement(select_amount: &SelectAmount,
                          table_name: &Box<dyn TableNameSupplier>,
                          where_clauses: &Option<Vec<QueryClause>>)
    -> String {

    let mut query = format!("SELECT {} FROM {}", select_amount.get(), table_name.extract_table_name());
    query = query + &build_where_clause_simple(where_clauses);
    query = query + &select_amount.get_additional();
    return query.to_string()
}

fn build_insert_statement(table_name: &Box<dyn TableNameSupplier>, field_names: &Vec<String>) -> String {
    let field_count = field_names.iter().count();
    // if no fields provided
    // return empty
    if field_count == 0{
        return "".to_string()
    }

    let query = format!("INSERT INTO {} ({}) VALUES ({})",
                            table_name.extract_table_name(),
                            build_fields_chaining(field_names),
                            build_values_chaining(field_count));
    return query
}

fn build_delete_statement(table_name: &Box<dyn TableNameSupplier>, where_clauses: &Option<Vec<QueryClause>>) -> String {
    return format!("DELETE FROM {}", table_name.extract_table_name()) + &build_where_clause_simple(where_clauses);
}

fn build_update_statement(table_name: &Box<dyn TableNameSupplier>,
                          field_names: &Vec<String>,
                          where_clauses: &Option<Vec<QueryClause>>)
                          -> String {

    let query = format!("UPDATE {} SET {}{}",
                            table_name.extract_table_name(),
                            build_update_setters(field_names),
                            build_where_clause(where_clauses, field_names.iter().count()));

    return query
}

// helpers
fn build_where_clause_simple(where_clauses:  &Option<Vec<QueryClause>>) -> String {
    return build_where_clause(where_clauses, 0);
}

fn build_where_clause(where_clauses:  &Option<Vec<QueryClause>>, existing_dynamic_params: usize) -> String{
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
                query = query + &query_clause.get(index + 1 + existing_dynamic_params) + " AND "
            } else {
                query = query + &query_clause.get(index + 1 + existing_dynamic_params)
            }
        }
    }
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


fn build_update_setters(field_names: &Vec<String>) -> String {
    let mut setters: String = String::default();
    for (index, field) in field_names.iter().enumerate() {
        setters = setters + &format!("{} = ${}", field, index+1);
        if index < field_names.iter().count() - 1{
            setters = setters + ", ";
        }
    }
    return setters
}

