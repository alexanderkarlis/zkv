use std::collections::HashMap;
use std::io::Write;
use std::vec::Vec;

#[derive(Debug,Clone)]
pub struct MemDb {
    pub path: std::string::String
}

/// * MemDb saves an array separated by newline (\n) of keys 
/// and values unassociated with the order of entry, and sorts 
/// them by keys *
impl MemDb {
    pub fn get(&self, search_key: &str) -> std::string::String {
       let bytes_array = std::fs::read(&self.path).unwrap();
       let str_map = std::str::from_utf8(&bytes_array).unwrap();

       // In a traditional k-v store, there would never be a key with multiple
       // values; currently, this is the portion of my book that I have been
       // going through in D.S. where we havent done find/replace?
       for pair in str_map.trim_end().split("\n") {
           let key: &str = pair.split(",").nth(0).expect("expected key");
           let value: &str = pair.split(",").nth(1).expect("expected value");
           if search_key == key {
               // return first instance of the key
               return value.to_string()
           }
       }
       String::from("")
    }

    pub fn put(&self, key: &str, value: &str) -> std::io::Result<()> {
        let mut map = HashMap::new();
        map.insert(key, value);
        // let ser_map = serde_json::to_string(&map).unwrap(); 
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(&self.path)?;
        let kv_str = format!("{},{}\n", key, value);
        file.write_all(kv_str.as_bytes())?;
        Ok(())
    }

    pub fn delete(&self) -> std::io::Result<()> {
        Ok(())
    }

    fn read_data_file(&self) -> HashMap<String, String> {
        
    }
}
