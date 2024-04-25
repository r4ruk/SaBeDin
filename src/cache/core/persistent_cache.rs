use serde::de::DeserializeOwned;
use serde_json::Value;

pub trait PersistentStorageHandler {
    fn get<TItem>(&self, key: &str) -> Option<TItem> where TItem: DeserializeOwned;
    fn append_element(element: Value);
    fn remove_element(key: String);
    fn reset_store();
    fn reload_store();
}


pub struct PersistentStorage{}

impl PersistentStorageHandler for PersistentStorage {

    /// gets a possible value from the available stores depending on the given key
    fn get<TItem>(&self, key: &str) -> Option<TItem>
        where
            TItem: DeserializeOwned {

        // retrieve information from fast storage
         // redis or textfile or something else?


        return None
    }

    fn append_element(element: Value) {
        // store element into fast storage... Redis, Textfile?
        todo!()
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