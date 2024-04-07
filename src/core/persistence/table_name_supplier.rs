use crate::core::persistence::table_names::{TABLE_NAMES, TableName};

/// TableName enums have to implement this, so it can be used boxed in the query builder
pub trait TableNameSupplier: Send + Sync{
    fn extract_table_name(&self) -> String;
}