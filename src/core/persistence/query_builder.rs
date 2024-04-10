use crate::core::persistence::table_name_supplier::TableNameSupplier;

pub struct PagingQuery {
    pub amount: i16,
    pub page_num: i16
}

#[allow(unused)]
pub enum Sorting {
    Ascending(Vec<String>),
    Descending(Vec<String>),
    Default
}

impl Sorting {
    /// method translates the Sorting into a valid SQL sorting query string
    fn translate(&self, ) -> String {
        return match self {
            Sorting::Ascending(column_names) => {
                let mut ascending_ordering = "ORDER BY ".to_string();
                for col in column_names {
                    ascending_ordering = ascending_ordering + col + ",";
                }
                // remove last char as it is a , which should not be at this point
                ascending_ordering.pop();
                ascending_ordering = ascending_ordering + " ";
                ascending_ordering.to_string()
            }
            Sorting::Descending(coulmn_names) => {
                let mut descending_ordering = "ORDER BY ".to_string();
                for col in coulmn_names {
                    descending_ordering = descending_ordering + col + " DESC,"
                }

                // removing last element as it is a , which should not be there
                descending_ordering.pop();
                descending_ordering = descending_ordering + " ";
                descending_ordering
            }
            // undefined sorting order should still be sorted as paging should always be on
            Sorting::Default => {
                "ORDER BY id ".to_string()
            }
        }
    }
}

#[allow(unused)]
pub struct OrderInformation {
    column_name: String,
    sorting: Sorting
}

impl PagingQuery {
    fn translate(&self) -> String {
        let offset = self.amount * self.page_num;
        return format!("LIMIT {} OFFSET {}", self.amount, offset)
    }
}

// TODO sooner or later think about that so that queries can be made with AND and OR chaining
// pub enum ChainingStrategy {
//     AND,
//     OR
// }

#[allow(unused)]
pub enum QueryBuilder {
    /// params: FromTableName, Option\<Vec\<QueryClause\>\>,Sorting, Option\<PagingQuery\>
    Select(Box<dyn TableNameSupplier>, Option<Vec<QueryClause>>, Sorting, Option<PagingQuery>),

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
            QueryBuilder::Select(table_name, where_clauses, sorting, paging_query) => build_select_statement(table_name, where_clauses, sorting, paging_query),
            QueryBuilder::Insert(table_name, field_names) => build_insert_statement(table_name, field_names),
            QueryBuilder::Update(table_name, field_names, where_clauses) => build_update_statement(table_name, field_names, where_clauses),
            QueryBuilder::Delete(table_name, where_clauses) => build_delete_statement(table_name, where_clauses),
        }
    }
}

fn build_select_statement(table_name: &Box<dyn TableNameSupplier>,
                          where_clauses: &Option<Vec<QueryClause>>,
                          sorting: &Sorting,
                          paging_query: &Option<PagingQuery>)
    -> String {

    let mut query = format!("SELECT * FROM {} ", table_name.extract_table_name());
    query = query + &build_where_clause_simple(where_clauses);
    query =  query + &sorting.translate();
    match paging_query {
        Some(paging) => query = query + &paging.translate(),
        None => ()
    }

    return query.trim_end().to_string()
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
    const DELETE_STATEMENT: &str = "DELETE FROM ";
    if where_clauses.is_none() {
        return format!("{}{}", DELETE_STATEMENT, table_name.extract_table_name())
    }
    return format!("{}{} ", DELETE_STATEMENT,table_name.extract_table_name()) + &build_where_clause_simple(where_clauses).trim_end();
}

fn build_update_statement(table_name: &Box<dyn TableNameSupplier>,
                          field_names: &Vec<String>,
                          where_clauses: &Option<Vec<QueryClause>>)
                          -> String {

    let query = format!("UPDATE {} SET {} {}",
                            table_name.extract_table_name(),
                            build_update_setters(field_names),
                            build_where_clause(where_clauses, field_names.iter().count()).trim_end());

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
        query = query + "WHERE ";
        for (index, query_clause) in wheres.iter().enumerate() {
            if index < where_count - 1 {
                query = query + &query_clause.get(index + 1 + existing_dynamic_params) + " AND "
            } else {
                query = query + &query_clause.get(index + 1 + existing_dynamic_params)
            }
        }
        query = query + " ";
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

