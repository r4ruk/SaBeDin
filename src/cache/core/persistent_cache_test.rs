use chrono::Utc;
use serde_json::json;
use crate::cache::core::persistent_cache::PersistentStorage;
use crate::cache::core::persistent_cache::PersistentStorageHandler;

#[test]
fn test_persistent() {
    // initialize
    let mut persistent_cache = PersistentStorage::initialize();

    // insert
    persistent_cache.insert("testkey".to_string(), (Utc::now(), json!("test")));

    assert!(persistent_cache.get("testkey").is_some());

    // update
    persistent_cache.insert("testkey".to_string(), (Utc::now(), json!("updated")));

    let updated_element = persistent_cache.get("testkey");
    assert!(persistent_cache.get("testkey").is_some());
    assert_eq!(*updated_element.unwrap().1.as_str().unwrap(), "updated".to_string())

    // TODO remove & Assert


}

//TODO reset, reload aso.


