use std::fmt::Debug;
use std::error::Error;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs;
use sha256::digest;
use std::ffi::OsStr;
use std::fs::metadata;
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
        //let check_dir = Path::new(path).read_dir()?;    //checks dir existence.
        fs::create_dir_all(&path)?;                 //creates dir at path. if error, returns std error.
        //TODO: should we exclude target from possible directories creation?
        let is_empty = Path::new(path).read_dir()?.next().is_none();
        println!("{}",is_empty);
        match is_empty {
            true => {                                   //no existing key-value mappings
                let new_kvstore = KVStore {
                    size: 0,
                    path: path.to_string(),
                };
                Ok(new_kvstore)
            },
            false => {  
                let mut counter = 0;
                for entry in fs::read_dir(path)? {      //grabs all entries in the directory and searches for ".key"
                    let entry = entry?;                 //counting all the KV pairs in the directory
                    //let filename = entry.file_name().into_string();   //to initialize a KVStore instance with an existing number of pairs
                    let pathname = entry.path();            //https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.path
                    let filename = pathname.to_str().unwrap();
                    let file_metadata = metadata(filename).unwrap();    //https://stackoverflow.com/questions/30309100/how-to-check-if-a-given-path-is-a-file-or-directory
                    if file_metadata.is_dir() {     //beginning of sub directory check for keyvalue pairs

                        for entry in fs::read_dir(filename)? {      
                            let entry = entry?;                 
                            let pathname = entry.path();            
                            let filename2 = pathname.to_str().unwrap();
                            if filename2.contains(&String::from(".key")) {
                                counter = counter + 1;       
                            }
                        }
                    }
                    //println!("{}",filename);

                    if filename.contains(&String::from(".key")) {
                        counter = counter + 1;          //sets counter for existing keyvalue pairs
                    }
                }
                //println!("{} is the counter",counter);
                let new_kvstore = KVStore {             //create instance of KVStore to account for existing and new key value pairs
                    size: counter,
                    path: path.to_string(),
                };
                Ok(new_kvstore)
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
        let serialize_value = serde_json::to_string(&value).unwrap();
        
        let hashed_key = digest(&serialize_key);
        let hashed_value = digest(&serialize_value);

        println!("hashed_key: {}, hashed_value: {}", hashed_key, hashed_value);
        
        let key_path = format!("{}{}{}", self.path, &hashed_key, &String::from(".key"));
        
        for entry in fs::read_dir(&self.path)? {      
            let entry = entry?;                 
            let pathname = entry.path();            
            let filename = pathname.to_str().unwrap();
            let file_metadata = metadata(filename).unwrap();    
            if file_metadata.is_dir() {     

                for entry in fs::read_dir(filename)? {      
                    let entry = entry?;                 
                    let pathname = entry.path();            
                    let sub_dir_filename = pathname.to_str().unwrap();
                    
                    if let sub_dir_filename = &*key_path {
                        println!("it worked!");
                    }
                }
            }

            if let filename = &*key_path {
                println!("it worked!");
            }
        }

        // Need to handle case where self.path is "." and if self.path does not contain "/"

        // let key_path = format!("{}{}{}", self.path, &hashed_key, &String::from(".key"));
        // fs::write(&key_path, serialize_key).expect("Unable to write file");

        // let value_path = format!("{}{}{}", self.path, &hashed_value, &String::from(".value"));
        // fs::write(&value_path, serialize_value).expect("Unable to write file");

        // println!("value_path: {}, key_path: {}", value_path, key_path);

        Ok(())
    }

    // fn lookup<K, V>(self: &Self, key: K) -> std::io::Result<V> {
    //     Ok()
    // }

    // fn remove<K, V>(self: &mut Self, key: K) -> std::io::Result<V> {
    //     Ok()
    // }
}