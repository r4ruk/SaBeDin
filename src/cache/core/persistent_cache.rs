use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions, remove_file};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::ops::Add;
use std::path::Path;
use chrono::{DateTime, Utc};
use serde_json::Value;
use crate::core::contracts::errors::GeneralServerError;
use crate::core::utils::file_helper;
use crate::logger::core_logger::{get_logger, LoggingLevel};

pub trait PersistentStorageHandler {
    fn get(&self, key: &str) -> Option<&(DateTime<Utc>, Value)>;
    fn insert(&mut self, key: String, element: (DateTime<Utc>, Value));
    fn remove_element(&mut self, key: String);
    fn reset_store(&self);
    fn reload_store(&mut self);
}


pub struct PersistentStorage{
    store: HashMap<String, (DateTime<Utc>, Value)>
}

/// Design decision for now is using textfile as i prefer not having more 3rd party dependencies
/// i have to handle at this point.
impl PersistentStorageHandler for PersistentStorage {

    /// gets a possible value from the available stores depending on the given key
    fn get(&self, key: &str) -> Option<&(DateTime<Utc>, Value)> {
        let result = self.store.get(key);
        return result
    }

    // inserts or updates (if existing) element
    fn insert(&mut self, key: String, element: (DateTime<Utc>, Value)) {
        if self.store.contains_key(&key) {
            self.store.insert(key, element);
            self.rewrite_full()
        } else {
            let mut newlines = HashMap::new();
            newlines.insert(key.clone(), element.clone());
            Self::write_lines(newlines);
            self.store.insert(key, element);
        }
    }

    fn remove_element(&mut self, key: String) {
        self.store.remove(&key);
        self.rewrite_full();
    }

    fn reset_store(&self) {
        let path_with_file = Self::get_cache_path();
        let result = remove_file(path_with_file);
        if result.is_err() {
            let logger = get_logger();
            logger.lock().unwrap().log_error(GeneralServerError { message: "Could not clear persistant cache".to_string() }, LoggingLevel::Error);
        }
    }

    fn reload_store(&mut self) {
        let path = Self::get_cache_path();
        let file = File::open(path);
        if Self::does_cachefile_exist() {
            let reader = BufReader::new(file.unwrap());
            self.store.clear();
            for line in reader.lines() {
                if let Some((key, value)) = line.unwrap().split_once(';') {
                    // Split successful, handle first and second parts
                    self.store.insert(key.to_string(), (Utc::now(), serde_json::from_str(value).unwrap()));
                } else {
                    let logger = get_logger();
                    logger.lock().unwrap().log_error(GeneralServerError{
                        message: "Could not parse the existing persistant cache. Resetting...".to_string()
                    }, LoggingLevel::Error);
                    self.reset_store()
                }
            }
        }
    }
}

impl PersistentStorage {
    pub fn initialize() -> Self {
        let mut persistent_store = PersistentStorage{
            store: Default::default(),
        };
        persistent_store.reload_store();
        return persistent_store
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
            if Self::does_cachefile_exist() {
                let result = remove_file(cachepath.clone());
                if result.is_ok() {
                    break;
                }
            } else {
                break
            }
        }
        // write line in any case here as it shouldve been purged
        Self::write_lines(self.store.clone());
    }

    fn write_lines(lines: HashMap<String, (DateTime<Utc>,Value)>) {
        let path_with_file = Self::get_cache_path();

        // commented on purpose, can be uncommented to get path information
        // println!("persistant file store: {}", path_with_file);

        let path_buf = Path::new(&path_with_file);

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path_buf);

        let mut file = BufWriter::new(file.unwrap());
        for line in lines {
            let result = writeln!(file, "{};{}",line.0, line.1.1.to_string());
            if let Err(e) = result {
                let logger = get_logger();
                logger.lock().unwrap().log_error(GeneralServerError {
                    message: format!("Could not write line into persistent cache. System Error {}", e)
                }, LoggingLevel::Error);
            }
        }
    }

    fn does_cachefile_exist() -> bool {
        let cachepath = Self::get_cache_path();
        let exists = match fs::metadata(&cachepath) {
            Ok(_) => {
                true
            }
            Err(err) => {
                let logger = get_logger();
                logger.lock().unwrap().log_error(GeneralServerError{message: err.to_string()}, LoggingLevel::Error);
                false
            }
        };
        return exists
    }

}

