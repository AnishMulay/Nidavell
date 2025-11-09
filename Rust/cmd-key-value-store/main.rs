mod kv;
use kv::KVStore;

fn main () {
    let mut store = KVStore::new();
    store.add(String::from("name"), String::from("Anish"));
}