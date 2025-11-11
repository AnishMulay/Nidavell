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

    pub fn run_command(&mut self, command: &String) {
        let words: Vec<&str> = command.trim().split_whitespace().collect();
        let mut operation: &str = "";

        // possible optimization here, look into production level code to understand how its done there
        match words.get(0).copied() {
            Some("set") => {
                let Some(keyString) = words.get(1).copied() else {todo!()};
                let Some(valString) = words.get(2).copied() else {todo!()};
                self.add(String::from(keyString), String::from(valString));
            },
            Some("get") => operation = "get",
            Some("delete") => operation = "delete",
            _ => operation = "invalid"
        }

        println!("operation {} was called", operation);
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
