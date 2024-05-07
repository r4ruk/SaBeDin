use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde_json::Value;
use crate::cache::core::persistent_cache::{PersistentStorage, PersistentStorageHandler};
use crate::core::persistence::query_builder::Sorting::Default;

pub struct Cache {
    // cache holding information for an hour
    short_store: Arc<Mutex<HashMap<String, ( DateTime<Utc>, Value)>>>,

    // store holding information for 8 hours
    mid_store: Arc<Mutex<HashMap<String, ( DateTime<Utc>, Value)>>>,

    persistent_store: Arc<Mutex<PersistentStorage>>

    // store holding information persistently (stored in an intermediate store
    // persistent_store: Arc<Mutex<HashMap<String, (DateTime<Utc>, Value)>>>
}

/// Lifetime definitions for store
pub enum StoreLifetime {
    Short,
    Mid,
    Persistent
}

impl Cache {
    /// Initializes the Cache
    pub fn initialize() -> Self {
        return Cache {
            short_store: Arc::new(Mutex::new(HashMap::new())),
            mid_store: Arc::new(Mutex::new(HashMap::new())),
            persistent_store: Arc::new(Mutex::new(PersistentStorage::initialize()))
        }
    }

    /// gets a possible value from the available stores depending on the given key
    pub fn get<TItem>(&self, key: &str) -> Option<TItem>
        where
            TItem: DeserializeOwned {

        // retrieving from store which is most up to date
        let element = self.get_from_cache(StoreLifetime::Short, key)
            .or_else(|| self.get_from_cache(StoreLifetime::Mid, key))
            .or_else(|| self.get_from_cache(StoreLifetime::Persistent, key));

        match element {
            Some(value) => {
                // debug print to see element which gets retrieved
                // println!("element: {:?}", value);
                Some(serde_json::from_value::<TItem>(value).unwrap())
            },
            None => return None
        }
    }

    /// method removes given key from all the stores which contain it,
    /// so it will be retrieved again from persistent storage next time it is requested
    pub fn invalidate_item(&mut self, key: &str) {
        self.short_store.lock().unwrap().remove(key);
        self.mid_store.lock().unwrap().remove(key);
    }

    /// Resets all the caches
    pub fn reset(&mut self) {
        self.short_store.lock().unwrap().clear();
        self.mid_store.lock().unwrap().clear();
        // reset_persistent_store();
    }

    // method adds element to cache (into default store which is short (1h))
    pub fn add_element(&mut self, item: (String, Value)) {
        self.add_element_internal(item, StoreLifetime::Short);
    }

    /// Method replaces element if it exists and otherwise inserts the given value
    pub fn update_element(&mut self, item: (String, Value), store_lifetime: StoreLifetime) {
        self.add_element_in_specific_store(item, store_lifetime)
    }

    // method to add element into specific store
    pub fn add_element_in_specific_store(&mut self, item: (String, Value), store_lifetime: StoreLifetime) {
        self.add_element_internal(item, store_lifetime);
    }

    // adds element internally into the defined store
    fn add_element_internal(&mut self, item: (String, Value), store_lifetime: StoreLifetime) {
        match store_lifetime {
            StoreLifetime::Short => {
                self.short_store.lock().unwrap().insert(item.0, (Utc::now(), item.1));
            },
            StoreLifetime::Mid => {
                self.mid_store.lock().unwrap().insert(item.0, (Utc::now(), item.1));
            },
            StoreLifetime::Persistent => {
                self.persistent_store.lock().unwrap().insert(item.0, (Utc::now(), item.1));
            }
        }
    }

    // gets element from cache
    fn get_from_cache(&self, from_lifetime: StoreLifetime, key: &str) -> Option<Value> {
        let result = match from_lifetime {
            StoreLifetime::Short => self.short_store.lock().unwrap().get(key).cloned(),
            StoreLifetime::Mid => self.mid_store.lock().unwrap().get(key).cloned(),
            StoreLifetime::Persistent => self.persistent_store.lock().unwrap().get(key).cloned()
        };

        match result {
            Some(value) =>{
                let (_, val) = value.clone();
                Some(val)
            }
            None => None,
        }
    }

    // start worker to check for elements in cache to clean up
    fn start_worker(&self) {
        let store_clone = self.short_store.clone();
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::new(600, 0));
                let mut map = store_clone.lock().unwrap();
                let current_time = Utc::now();
                map.retain(|_, (expiry, _)| *expiry > current_time);
            }
        });
    }
}