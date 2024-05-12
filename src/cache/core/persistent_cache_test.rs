use chrono::Utc;
use serde_json::json;
use crate::cache::core::persistent_cache::PersistentStorage;
use crate::cache::core::persistent_cache::PersistentStorageHandler;

fn cleanup_test(store: PersistentStorage) {
    store.reset_store();
}

#[test]
fn basic_persistent_cache_test() {

    // initialize
    let mut persistent_cache = PersistentStorage::initialize();

    // insert
    persistent_cache.insert("testkey".to_string(), (Utc::now(), json!("test")));

    assert!(persistent_cache.get("testkey").is_some());

    // update
    persistent_cache.insert("testkey".to_string(), (Utc::now(), json!("updated")));

    let updated_element = persistent_cache.get("testkey");
    assert!(persistent_cache.get("testkey").is_some());
    assert_eq!(*updated_element.unwrap().1.as_str().unwrap(), "updated".to_string());

    persistent_cache.remove_element("testkey".to_string());

    assert!(persistent_cache.get("testkey").is_none());

    cleanup_test(persistent_cache);
}
