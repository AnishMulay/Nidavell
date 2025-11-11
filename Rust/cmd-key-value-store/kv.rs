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
            Some("get") => {
                let Some(keyString) = words.get(1).copied() else {todo!()};
                self.get(String::from(keyString));
            }
            Some("delete") => {
                let Some(keyString) = words.get(1).copied() else {todo!()};
                self.delete(String::from(keyString));
            },
            _ => {
                println!("Invalid operation")
            }
        }

        println!()
    }

    pub fn add(&mut self, key: String, val: String) {
        println!("key {} added", key);
        self.mp.insert(key, val);
    }

    pub fn get(&self, key: String) {
        if let Some(val) = self.mp.get(&key) {
            println!("{}: {}", key, val);   
        }
        else {
            println!("key {} not found", key);
        }
    }

    pub fn delete(&mut self, key: String) {
        self.mp.remove(&key);
        println!("key {} deleted", key)
    }
}
