mod kv;
use kv::KVStore;
use std::io;

fn main () {
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("Failed to read line");
    println!("You entered the following command -> {}", command)
}
