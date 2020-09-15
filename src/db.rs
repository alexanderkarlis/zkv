use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct MemDb {
    pub path: std::string::String
}

impl MemDb {
    pub fn get(&self, _key: &str) {
       let bytes_array = std::fs::read(&self.path).unwrap();
       println!("{:#?}", bytes_array);
       let str_map = std::str::from_utf8(&bytes_array).unwrap();
       println!("{:#?}", str_map);
       let d_map = serde_json::from_str(&str_map).unwrap(); 
       println!("{:#?}", d_map);
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
