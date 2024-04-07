use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::core::persistence::table_names::TableNameSupplier;


/// enum representing different entities stored in the database
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum TableNamePortfolio {
    Article,
}

impl TableNameSupplier for TableNamePortfolio {
    fn extract_table_name(&self) -> String {
        return if let Some(value) = TABLE_NAMES.get(self) {
            value.clone()
        } else {
            "".to_string()
        }
    }
}

lazy_static!{
    pub static ref TABLE_NAMES: HashMap<TableNamePortfolio, String> = {
        let mut m = HashMap::new();

        // extend the following lines with matching database informations
        m.insert(TableNamePortfolio::Article, "articles".to_string());


        m
    };
}
