use std::collections::HashMap;
use lazy_static::lazy_static;

pub trait TableNameSupplier: Send + Sync{
    fn extract_table_name(&self) -> String;
}

/// enum representing different entities stored in the database
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum TableName {
    Users,
}

impl TableNameSupplier for TableName {
    fn extract_table_name(&self) -> String {
        return if let Some(value) = TABLE_NAMES.get(self) {
            value.clone()
        } else {
            "".to_string()
        }
    }
}

lazy_static!{
    pub static ref TABLE_NAMES: HashMap<TableName, String> = {
        let mut m = HashMap::new();

        // extend the following lines with matching database informations
        m.insert(TableName::Users, "users".to_string());


        m
    };
}
