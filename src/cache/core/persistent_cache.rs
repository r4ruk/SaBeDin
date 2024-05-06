use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions, remove_file};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::ops::Add;
use std::path::Path;
use serde::de::DeserializeOwned;
use serde_json::Value;
use crate::core::utils::file_helper;
use crate::core::utils::utils::get_os_newline;

pub trait PersistentStorageHandler {
    fn get<TItem>(&self, key: &str) -> Option<TItem> where TItem: DeserializeOwned;
    fn append_element(&mut self,key: String, element: Value);
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

        // todo read from store itself which should have actual data always loaded.

        return None
    }

    fn append_element(&mut self, key: String, element: Value) {
        if self.store.contains_key(&key) {
            self.store.insert(key, element);
            self.rewrite_full()
        } else {
            let mut newlines = HashMap::new();
            newlines.insert(key, element);
            Self::write_lines(newlines);
        }
    }

    fn remove_element(&mut self, key: String) {
        self.store.remove(&key);
        self.rewrite_full();
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

    fn rewrite_full(&self) {
        let cachepath = Self::get_cache_path();
        // loop to retry if file is not closed at that moment
        loop {
            let result = remove_file(cachepath.clone());
            if result.is_ok(){
                break
            }
        }
        self.rewrite_full();
    }

    fn write_lines(lines: HashMap<String, Value>) {
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
        for line in lines {
            let result = writeln!(file, "{};{}",line.0, line.1.to_string());
            if result.is_err() {
                // TODO add logger functionality
            }
        }
    }
}

