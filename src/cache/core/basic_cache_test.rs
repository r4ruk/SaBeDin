use std::collections::HashMap;
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;
use crate::cache::core::basic_cache::{Cache, StoreLifetime};
use crate::cache::core::persistent_cache::PersistentStorage;
use crate::example::portfolio::contracts::article::Article;
use crate::cache::core::persistent_cache::PersistentStorageHandler;

/// function used to initialize the basic testcase
#[allow(unused)]
fn initialize_basic_cache_testcase() -> (Article, Cache) {
    let mut element = Article {
        id: Uuid::new_v4(),
        programming_key_name: "ProgrammingKeyName".to_string(),
        title: "TestTitel".to_string(),
        contents: "Dies ist ein Inhalt".to_string(),
        tags: "test, ueli, hans".to_string(),
        created_at: Utc::now(),
    };

    let mut cache = Cache::initialize();
    (element, cache)
}


#[test]
fn test_adding_element() {
    let (element, mut cache) = initialize_basic_cache_testcase();
    let cache_key = "testarticle".to_string();

    cache.add_element((cache_key.clone(), serde_json::to_value(&element).unwrap()));

    let stored_cache_item = cache.get::<Article>(&cache_key);

    let stored_element = stored_cache_item.unwrap_or_else(|| Article {
        id: Default::default(),
        programming_key_name: "".to_string(),
        title: "".to_string(),
        contents: "".to_string(),
        tags: "".to_string(),
        created_at: Default::default(),
    });

    assert_eq!(element.id, stored_element.id);
    assert_eq!(element.programming_key_name, stored_element.programming_key_name);
    assert_eq!(element.title, stored_element.title);
    assert_eq!(element.contents, stored_element.contents);
    assert_eq!(element.tags, stored_element.tags);
    assert_eq!(element.created_at, stored_element.created_at);
}

#[test]
fn test_update_item_in_cache() {
    let (element, mut cache) = initialize_basic_cache_testcase();
    let cache_key = "testarticle".to_string();
    cache.add_element((cache_key.clone(), serde_json::to_value(&element).unwrap()));

    let initial_element = cache.get::<Article>(&cache_key);

    assert_eq!(initial_element.is_some(), true);

    let updated_element = Article {
        id: Uuid::new_v4(),
        programming_key_name: element.clone().programming_key_name + "_updated",
        title: element.clone().title + "_updated",
        contents: element.clone().contents + "_updated",
        tags: element.clone().tags + "_updated",
        created_at: Utc::now(),
    };

    cache.update_element((cache_key.clone(), serde_json::to_value(&updated_element).unwrap()), StoreLifetime::Short);
    let retrieved_updated = cache.get::<Article>(&cache_key).unwrap();

    assert_eq!(updated_element.id, retrieved_updated.id);
    assert_eq!(updated_element.programming_key_name, retrieved_updated.programming_key_name);
    assert_eq!(updated_element.title, retrieved_updated.title);
    assert_eq!(updated_element.contents, retrieved_updated.contents);
    assert_eq!(updated_element.tags, retrieved_updated.tags);
    assert_eq!(updated_element.created_at, retrieved_updated.created_at);
}


#[test]
fn test_cleanup_cache() {
    let (element, mut cache) = initialize_basic_cache_testcase();
    let cache_key = "testarticle".to_string();
    cache.add_element((cache_key.clone(), serde_json::to_value(&element).unwrap()));

    let stored_cache_item = cache.get::<Article>(&cache_key);

    assert_eq!(stored_cache_item.is_some(), true);

    cache.reset();
    let none_element = cache.get::<Article>(&cache_key);
    assert_eq!(none_element.is_none(), true)
}

#[test]
fn test_invalidate_item_cache() {
    let (element, mut cache) = initialize_basic_cache_testcase();
    let cache_key = "testarticle".to_string();
    cache.add_element((cache_key.clone(), serde_json::to_value(&element).unwrap()));

    let stored_cache_item = cache.get::<Article>(&cache_key);

    assert_eq!(stored_cache_item.is_some(), true);

    cache.invalidate_item(&cache_key);
    let none_element = cache.get::<Article>(&cache_key);
    assert_eq!(none_element.is_none(), true)
}
