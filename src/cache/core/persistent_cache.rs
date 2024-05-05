use std::collections::HashMap;
use std::fs::{File, OpenOptions, remove_file};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::ops::Add;
use std::path::Path;
use serde::de::DeserializeOwned;
use serde_json::Value;
use crate::core::utils::file_helper;

pub trait PersistentStorageHandler {
    fn get<TItem>(&self, key: &str) -> Option<TItem> where TItem: DeserializeOwned;
    fn append_element(&self,key: String, element: Value);
    fn remove_element(&self, key: String);
    fn reset_store(&self);
    fn reload_store(&mut self);
}


pub struct PersistentStorage{
    store: HashMap<String, Value>
}

/// Design decision for now is using textfile as i prefer not having more 3rd party dependencies
/// i have to handle at this point.
impl PersistentStorageHandler for PersistentStorage {

    /// gets a possible value from the available stores depending on the given key
    fn get<TItem>(&self, key: &str) -> Option<TItem>
        where
            TItem: DeserializeOwned {

         // redis or textfile or something else?


        return None
    }

    fn append_element(&self, key: String, element: Value) {
        let path_with_file = Self::get_cache_path();
        println!("persistant file store: {}", path_with_file);
        let path_buf = Path::new(&path_with_file);

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path_buf);

        let mut file = BufWriter::new(file.unwrap());
        let result = writeln!(file, "{};{}",key, element.to_string());

        if result.is_err() {
            // TODO Add logger finally now.
        }
    }

    fn remove_element(&self, key: String) {
        todo!()
        // todo probably just rewrite full cache with removed element from hashmap
    }

    fn reset_store(&self) {
        let path_with_file = Self::get_cache_path();
        remove_file(path_with_file).unwrap();
    }

    fn reload_store(&mut self) {
        let path = Self::get_cache_path();
        let file = File::open(path);
        let reader = BufReader::new(file.unwrap());
        self.store.clear();
        for line in reader.lines() {
            if let Some((key, value)) = line.unwrap().split_once(';') {
                // Split successful, handle first and second parts
                self.store.entry(key.parse().unwrap()).or_insert(value.parse().unwrap());
            } else {
                println!("Could not split persisted cache element");
            }
        }
    }
}

impl PersistentStorage {
    pub fn initialize() -> Self {
        PersistentStorage{
            store: Default::default(),
        }
    }
    fn get_cache_path() -> String {
        let dir = file_helper::get_temp();
        let path_with_file = dir.add("persistent_cache.txt");
        path_with_file
    }
}
