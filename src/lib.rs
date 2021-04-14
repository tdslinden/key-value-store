use std::fmt::Debug;
use std::error::Error;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs;


#[derive(Serialize, Deserialize, Debug)]
/// A struct that represents a key-value store.
pub struct KVStore {
    /// The number of key-value mappings currently stored.
    size: usize,
    /// The location of the file system where key-value mappings are stored.
    path: String,
}

/// A trait that defines the operations that need to be supported.
pub trait Operations {
    /// A function that initializes a KVStore instance.
    ///
    /// If there is no directory at the provided path, this should create it. If there is an error
    /// while creating a directory, this should return an [std::io::Error].
    ///
    /// If there are **no** key-value mappings stored already under the directory, this
    /// should simply create a new KVStore instance that can store and retrieve key-value mappings
    /// using the directory. It should also correctly initialize the size to 0.
    ///
    /// If there **are** existing key-value mappings stored already under the directory, this
    /// should initialize a KVStore instance that is able to store and retrieve existing key-value
    /// mappings as well as new key-value mappings. It should also correctly initialize the size to
    /// the number of existing key-value mappings.
    fn new(path: &str) -> std::io::Result<Self>
    where
        Self: Sized;

    /// A function that returns the number of key-value mappings currently stored.
    fn size(self: &Self) -> usize;

    /// A function that inserts a new key-value mapping.
    ///
    /// If there is **no** key-value mapping stored already with the same key, it should return
    /// `Ok(())` if storing is successfully done.
    ///
    /// If there **is** a key-value mapping stored already with the same key, it should return an
    /// [std::io::Error].
    ///
    /// Make sure you read and understand the assignment document regarding how to store key-value
    /// mappings using files as well as how to structure sub-directories.
    ///
    /// Make sure you understand what the trait bounds mean for K and V.
    ///
    /// Refer to [https://docs.serde.rs/serde/](https://docs.serde.rs/serde/)
    /// and [https://serde.rs](https://serde.rs) for serde.
    fn insert<K, V>(self: &mut Self, key: K, value: V) -> std::io::Result<()>
    where
        K: serde::Serialize + Default + Debug,
        V: serde::Serialize + Default + Debug;
}

impl Operations for KVStore {
    fn new(path: &str) -> std::io::Result<Self> {
        let check_dir = Path::new(path).read_dir()?; //returns std::err if cannot create
        
        let is_empty = Path::new(path).read_dir()?.next().is_none();
        match is_empty {
            True => {                                   //no existing key-value mappings
                let new_kvstore = KVStore {
                    size: 0,
                    path: path.to_string(),
                };
                Ok(new_kvstore)
            },
            False => {  //TODO HOW WE GRAB THE EXISTING KVS???/
                let check_existing_kvs = KVStore::new(&path.to_string());
                //possible idea: count all the .keys to find out how many KV pairs there are 
                //in the existing KV instance. 
                //TEMP
                let new_kvstore = KVStore {
                    size: 0,
                    path: path.to_string(),
                };
                Ok(new_kvstore)
                //TEMP
            }
        }
        
    }

    fn size(self: &Self) -> usize {
        self.size
    }

    fn insert<K, V>(self: &mut Self, key: K, value: V) -> std::io::Result<()>
    where
        K: serde::Serialize + Default + Debug,
        V: serde::Serialize + Default + Debug,
        {
        //serde a key, create a SHA for the key and the value, use filewriter to store in given path directory
        
        let serialize_key = serde_json::to_string(&key).unwrap();
        Ok(())
    }

    // fn lookup<K, V>(self: &Self, key: K) -> std::io::Result<V> {
    //     Ok()
    // }

    // fn remove<K, V>(self: &mut Self, key: K) -> std::io::Result<V> {
    //     Ok()
    // }
}