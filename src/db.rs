use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct MemDb {
    pub path: std::string::String
}

impl MemDb {
    pub fn get(&self, key: &str) -> std::string::String {
       let bytes_array = std::fs::read(&self.path).unwrap();
       let str_map = std::str::from_utf8(&bytes_array).unwrap();
       let d_map: HashMap<String, String> = serde_json::from_str(&str_map).unwrap(); 
       d_map.get(key).unwrap().to_string()
    }

    pub fn put(&self, key: &str, value: &str) -> std::io::Result<()> {
        let mut map = HashMap::new();
        map.insert(key, value);
        let ser_map = serde_json::to_string(&map).unwrap(); 
        println!("{:#?}", &self.path);
        std::fs::write(&self.path, ser_map)?;
        Ok(())
    }

    pub fn delete(&self) {}
}
