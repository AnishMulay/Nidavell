mod kv;
use kv::KVStore;
use std::io;

fn main () {
    let mut store: KVStore = KVStore::new();
    store.recover_using_log_file();

    let mut command: String = String::new();
    loop {
        command.clear();
        io::stdin().read_line(&mut command).expect("Failed to read line");
        if command.trim() == String::from("done") {
            break;
        } else {
            store.run_command(&command);
        }
    }
}
