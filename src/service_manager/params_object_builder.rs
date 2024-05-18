use std::collections::HashMap;
use crate::core::contracts::query_params::QueryOptions;
use crate::core::persistence::query_builder::Sorting;

pub fn build_query_options_from_params(params: HashMap<String, String>) -> Option<QueryOptions> {
    // queries will be added by function which is handling the called get function
    // but sorting and paging should be changeable in query params

    // initializing queryOptions
    let mut query_option: QueryOptions = Default::default();

    // paging params
    if let Some(val) = params.get("amountofitems") {
        let value_int: Result<i16, _> = val.parse();
        if value_int.is_ok() {
            query_option.paging_information.amount_of_items = value_int.unwrap()
        }
    }
    if let Some(val) = params.get("pagenum") {
        let value_int: Result<i16, _> = val.parse();
        if value_int.is_ok() {
            query_option.paging_information.page_num = value_int.unwrap()
        }
    }

    // Sorting params
    if let Some(val) = params.get("sortbyasc") {
        query_option.sorting_information = Sorting::Ascending(vec![val.to_string()]);
    }
    else if let Some(val) = params.get("sortbydesc") {
        query_option.sorting_information = Sorting::Descending(vec![val.to_string()]);
    }

    return Some(query_option)
}