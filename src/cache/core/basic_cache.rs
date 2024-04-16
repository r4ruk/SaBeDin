use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde_json::Value;

pub struct Cache {
    // cache holding information for an hour
    short_store: HashMap<String, ( DateTime<Utc>, Value)>,

    // store holding information for 8 hours
    mid_store: HashMap<String, ( DateTime<Utc>, Value)>,

    // store holding information persistently (stored in an intermediate store,
    // loading and storing on Startup/Shutdown)
    persistent_store: HashMap<String, (DateTime<Utc>, Value)>
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
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.get_from_cache(StoreLifetime::Short, key)
            .or_else(|| self.get_from_cache(StoreLifetime::Mid, key))
            .or_else(|| self.get_from_cache(StoreLifetime::Persistent, key))
    }


    // short store implementations


    // mid store implementations


    // persistent store implementations



    fn get_from_cache(&self, from_lifetime: StoreLifetime, key: &str) -> Option<&Value> {
        let mut result = match from_lifetime {
            StoreLifetime::Short => self.short_store.get(key),
            StoreLifetime::Mid => self.mid_store.get(key),
            StoreLifetime::Persistent => self.persistent_store.get(key)
        };

        return match result {
            Some(value) => {
                Some(&value.1)
            },
            None => {
                None
            }
        }
    }
}