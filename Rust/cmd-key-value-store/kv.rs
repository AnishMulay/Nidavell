use std::collections::HashMap;

pub struct KVStore {
    mp: HashMap<String, String>
}

pub struct CommandParser {}

impl CommandParser {
    pub fn parse_command(command: String) -> 
}

impl KVStore {
    pub fn new() -> Self {
        Self {
            mp: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, val: String) {
        println!("key {} added", key);
        self.mp.insert(key, val);
    }

    pub fn get(&self, key: String) -> Option<&String> {
        self.mp.get(&key)
    }

    pub fn remove(&mut self, key: String) {
        self.mp.remove(&key);
        println!("key {} removed", key)
    }
}
