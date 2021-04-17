#![no_main]
use libfuzzer_sys::fuzz_target;
use kvstore::{KVStore, Operations};

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(key) = std::str::from_utf8(data) {
        let mut kvs = KVStore::new(".").unwrap();
        kvs.insert(String::from(key), key).unwrap();
        kvs.lookup::<String, String>(String::from(key)).unwrap();
        kvs.remove::<String, String>(String::from(key)).unwrap();
    }
});