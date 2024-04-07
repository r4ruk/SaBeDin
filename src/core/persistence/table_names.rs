use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::core::persistence::table_name_supplier::TableNameSupplier;

/// enum representing different entities stored in the database
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum TableName {
    Users,
}

lazy_static!{
    pub static ref TABLE_NAMES: HashMap<TableName, String> = {
        let mut m = HashMap::new();

        // extend the following lines with matching database informations
        m.insert(TableName::Users, "users".to_string());


        m
    };
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
