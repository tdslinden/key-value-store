//! This is just an example.
use kvstore::{KVStore, Operations};

fn main() {
    let mut kvs = KVStore::new(".").unwrap();
    // kvs.insert(String::from("key"), 1 as i32).unwrap();
    // kvs.lookup::<String, i32>(String::from("key")).unwrap();
    // kvs.remove::<String, i32>(String::from("key")).unwrap();
}