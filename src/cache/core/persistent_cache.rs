use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::ops::Add;
use std::path::Path;
use serde::de::DeserializeOwned;
use serde_json::Value;
use crate::core::utils::file_helper;

pub trait PersistentStorageHandler {
    fn get<TItem>(&self, key: &str) -> Option<TItem> where TItem: DeserializeOwned;
    fn append_element(&self, element: Value);
    fn remove_element(key: String);
    fn reset_store();
    fn reload_store();
}


pub struct PersistentStorage{}

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

    fn append_element(&self, element: Value) {
        let dir = file_helper::get_temp();
        let path_with_file = dir.add("persistent_cache.txt");
        println!("persistant file store: {}", path_with_file);
        let path_buf = Path::new(&path_with_file);

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path_buf);

        let mut file = BufWriter::new(file.unwrap());
        let result = writeln!(file, "{}", element.to_string());

        if result.is_err() {
            // TODO Add logger finally now.
        }
    }

    fn remove_element(key: String) {
        todo!()
    }

    fn reset_store() {
        todo!()
    }

    fn reload_store() {
        todo!()
    }
}
