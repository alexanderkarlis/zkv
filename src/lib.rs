pub mod zkv {
    pub fn init(path: &str) -> std::io::Result<()> {
        let split_v: Vec<&str> = path.split("/").collect();
        if split_v.len() > 1 {
            let msg: std::string::String = format!("cannot init db here. {}", path);
            panic!(msg)
        }

        let mut db_path: std::string::String = String::from("./db/");
        db_path.push_str(path);
        std::fs::create_dir_all(db_path)?;
        Ok(())
    }

    #[test]
    fn test_init() {
        // self.init("data")

    }
}

pub mod test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test() {
        assert_ne!(2+2, 5);
    }
}