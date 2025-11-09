use std::collections::HashMap;

pub struct KVStore {
    mp: HashMap<String, String>
}

impl KVStore {
    pub fn new() -> Self {
        Self {
            mp: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, val: String) -> String {
        self.mp.insert(String::from(key), String::from(val));
        String::from("done")
    }
}