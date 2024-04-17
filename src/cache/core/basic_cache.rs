use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use serde_json::Value;
use time::Duration;

pub struct Cache {
    // cache holding information for an hour
    short_store: Arc<Mutex<HashMap<String, ( DateTime<Utc>, Value)>>>,

    // store holding information for 8 hours
    mid_store: Arc<Mutex<HashMap<String, ( DateTime<Utc>, Value)>>>,

    // store holding information persistently (stored in an intermediate store,
    // loading and storing on Startup/Shutdown)
    persistent_store: Arc<Mutex<HashMap<String, (DateTime<Utc>, Value)>>>
}

enum StoreLifetime {
    Short,
    Mid,
    Persistent
}


impl Cache {
    pub fn initialize() -> Self {
        return Cache {
            short_store: Default::default(),
            mid_store: Default::default(),
            persistent_store: Default::default(),
        }
    }

    // gets a possible value from the available stores depending on the given key
    pub fn get(&self, key: &str) -> Option<Value> {
        self.get_from_cache(StoreLifetime::Short, key)
            .or_else(|| self.get_from_cache(StoreLifetime::Mid, key))
            .or_else(|| self.get_from_cache(StoreLifetime::Persistent, key))
    }

    // method adds element to cache (into default store which is short (1h))
    pub fn add_element(&mut self, item: (String, Value)) {
        self.add_element_internal(item, StoreLifetime::Short);
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
            StoreLifetime::Persistent => self.persistent_store.lock().unwrap().get(key).cloned(),
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
                std::thread::sleep(std::time::Duration::new(1, 0));
                let mut map = store_clone.lock().unwrap();
                let current_time = Utc::now();
                map.retain(|_, (expiry, _)| *expiry > current_time);
            }
        });
    }
}