use serde_json::json;
use uuid::Uuid;
use crate::cache::core::basic_cache::Cache;
use crate::example::portfolio::contracts::article::Article;

#[test]
fn test_adding_element() {
    let mut element = Article {
        id: Uuid::new_v4(),
        programming_key_name: "ProgrammingKeyName".to_string(),
        title: "TestTitel".to_string(),
        contents: "Dies ist ein Inhalt".to_string(),
        tags: "test, ueli, hans".to_string(),
        created_at: Default::default(),
    };

    let mut cache = Cache::initialize();
    
    cache.add_element(("testarticle".to_string(), json!(element)))


    // TODO add finish of the test

    // assert_eq!(query_string, query.build_query());
}