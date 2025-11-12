use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

pub struct KVStore {
    mp: HashMap<String, String>,
    log_file: File
}

impl KVStore {
    pub fn new() -> Self {
        let Ok(lf) = OpenOptions::new().create(true).write(true).open("./kvstore/log") else {todo!()};
        println!("successfully opened log file");
        Self {
            mp: HashMap::new(),
            log_file: lf,
        }
    }

    pub fn run_command(&mut self, command: &String) {
        let words: Vec<&str> = command.trim().split_whitespace().collect();

        // possible optimization here, look into production level code to understand how its done there
        match words.get(0).copied() {
            Some("set") => {
                let Some(key_string) = words.get(1).copied() else {todo!()};
                let Some(val_string) = words.get(2).copied() else {todo!()};
                self.add(String::from(key_string), String::from(val_string));
            },
            Some("get") => {
                let Some(key_string) = words.get(1).copied() else {todo!()};
                self.get(String::from(key_string));
            }
            Some("delete") => {
                let Some(key_string) = words.get(1).copied() else {todo!()};
                self.delete(String::from(key_string));
            },
            _ => {
                println!("Invalid operation")
            }
        }

        println!()
    }

    pub fn add(&mut self, key: String, val: String) {
        println!("key {} added", key);
        writeln!(self.log_file, "add {} {}", key, val);
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
        writeln!(self.log_file, "delete {}", key);
        println!("key {} deleted", key)
    }
}
