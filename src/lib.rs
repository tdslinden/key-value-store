use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs;
use sha256::digest;
use std::fs::metadata;
use std::io::{Error, ErrorKind};


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

fn create_file_path<'a>(path: &String, hashed_value: &'a str, extension: &'a str) -> String {
    let file_path = match path.as_str() {
        "." => format!("{}{}{}", "/", &hashed_value, extension),
        _ => format!("{}{}{}{}", path, "/", &hashed_value, extension),
    };

    file_path
}

fn create_file_name<'a>(hashed_value: &'a str, extension: &'a str) -> String {
    format!("{}{}", &hashed_value, extension)
}

impl Operations for KVStore {
    fn new(path: &str) -> std::io::Result<Self> {
        //let check_dir = Path::new(path).read_dir()?;    //checks dir existence.
        fs::create_dir_all(&path)?;                 //creates dir at path. if error, returns std error.
        //TODO: should we exclude target from possible directories creation?
        let is_empty = Path::new(path).read_dir()?.next().is_none();
        //println!("{}",is_empty);
        
        let mut sanitized_path = String::from(path);    //will we need to add a / to the end of the path? 
        let length = sanitized_path.len();
        let last_char = &sanitized_path[length-1..];    //https://stackoverflow.com/questions/48642342/how-to-get-the-last-character-of-a-str
        //println!("{}",last_char);
        if !last_char.contains(&String::from("/")){     //if it does not contain a /, it will need to be added to the sanitized path
            sanitized_path = sanitized_path + "/";
        }
        match is_empty {
            true => {                                   //no existing key-value mappings
                let new_kvstore = KVStore {
                    size: 0,
                    path: sanitized_path,
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
                    println!("{}",filename);
                }
                println!("{} is the counter",counter);
                let new_kvstore = KVStore {             //create instance of KVStore to account for existing and new key value pairs
                    size: counter,
                    path: sanitized_path,
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
        let key_file_name = create_file_name(&hashed_key, ".key");
        let first_ten_key = &hashed_key[0..10];
        let desired_subdirectory_path = create_file_name(&self.path, &first_ten_key);

        let mut is_inserted = false;
        // check if a directory with first 10 characters from SHA exists
        for subdirectory in fs::read_dir(&self.path)? {      
            let subdirectory = subdirectory?;                 
            let path_name = subdirectory.path();            
            let subdirectory_path = path_name.to_str().unwrap();
            
            let subdirectory_name = path_name.file_name().unwrap().to_str().unwrap();
            let subdir_ten_key = &subdirectory_name[0..10];
            let file_metadata = metadata(subdirectory_path).unwrap(); 
                        
            if first_ten_key.eq(subdir_ten_key) {
                if file_metadata.is_dir() {
                    for entry in fs::read_dir(subdirectory_path)? {      
                        let entry = entry?;                 
                        let path_name = entry.path();            
                        let file_name = path_name.file_name().unwrap().to_str().unwrap();
                        
                        if file_name.eq(&key_file_name) {
                            let custom_error = Error::new(ErrorKind::AlreadyExists, "There is a key-value mapping stored already with the same key.");
                            return Err(custom_error);
                        } 
                    }
                }
                
                let key_file_path = create_file_path(&desired_subdirectory_path, &hashed_key, ".key");
                let value_file_path = create_file_path(&desired_subdirectory_path, &hashed_key, ".value");
                fs::write(&key_file_path, &serialize_key).expect("Unable to write file");
                fs::write(&value_file_path, &serialize_value).expect("Unable to write file");
                is_inserted = true;
            }
        }
        // if here, then create sub dir with first 10 chars and then write or write to existing 

        if !is_inserted {
            fs::create_dir(&desired_subdirectory_path)?;
            let key_file_path = create_file_path(&desired_subdirectory_path, &hashed_key, ".key");
            let value_file_path = create_file_path(&desired_subdirectory_path, &hashed_key, ".value");
            fs::write(&key_file_path, serialize_key).expect("Unable to write file");
            fs::write(&value_file_path, serialize_value).expect("Unable to write file");  
        }
        Ok(())
    }

    // fn lookup<K, V>(self: &Self, key: K) -> std::io::Result<V> {
    //     Ok()
    // }

    // fn remove<K, V>(self: &mut Self, key: K) -> std::io::Result<V> {
    //     Ok()
    // }
}
