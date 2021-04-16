//! This is just an example.
use kvstore::{KVStore, Operations};

fn main() {
    // Default
    let mut kvs = KVStore::new(".").unwrap();
    kvs.insert(String::from("key"), 1 as i32).unwrap();
    kvs.lookup::<String, i32>(String::from("key")).unwrap();
    kvs.remove::<String, i32>(String::from("key")).unwrap();

    // let mut kvs = KVStore::new(".").unwrap();
    // // kvs.insert(String::from("1"), 1 as i32).unwrap();
    // // kvs.insert(String::from("2"), 2 as i32).unwrap();
    // // kvs.insert(String::from("3"), 3 as i32).unwrap();
    // // kvs.insert(String::from("4"), 4 as i32).unwrap();
    // // kvs.insert(String::from("5"), 5 as i32).unwrap();
    // // println!("{:?}", kvs.lookup::<String, i32>(String::from("1")).unwrap());
    // kvs.remove::<String, i32>(String::from("1")).unwrap();
    // kvs.remove::<String, i32>(String::from("2")).unwrap();
    // kvs.remove::<String, i32>(String::from("3")).unwrap();
    // kvs.remove::<String, i32>(String::from("4")).unwrap();
    // kvs.remove::<String, i32>(String::from("5")).unwrap();
}