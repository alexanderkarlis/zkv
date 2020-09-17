use crate::utils;
use crate::db::MemDb;
use String;


#[derive(Debug, Clone)]
pub struct Session {
    pub path: String,
    pub sst_path: String,
    log_file: String,
    current_file: String,
    pub zkv: MemDb
}

impl Session {
    /// # Create Session Files
    /// *This function is designed create the necessary files*
    /// *in order to adequately start the zkv*
    ///
    /// * LOG => logs all transactions to/from session
    /// * CURRENT - Text  => current manifest to read
    pub fn init(mut self) -> Session {
        let mut p_log = self.path.clone();
        let mut p_current = self.path.clone();
        let mut p_sst = self.path.clone();

        p_log.push_str("/LOG");
        p_current.push_str("/CURRENT");
        p_sst.push_str("/DATA.sst");

        self.log_file = p_log;
        self.current_file = p_current;
        self.sst_path = p_sst;
        self
    }

    fn mk_files(&self) -> std::io::Result<()> {
        self.create_log_file();
        self.create_current_file();
        self.create_sst_file();
        Ok(())
    }

    fn create_log_file(&self) {
        let log_path: &str = &self.log_file;
        println!("{}", &self.log_file);
        std::fs::File::create(log_path).unwrap();
    }

    fn create_sst_file(&self) {
        println!("{:#?}", &self);
        std::fs::File::create(&self.sst_path.to_string()).unwrap();
    }

    fn create_current_file(&self) {
        let current_path: &str = &self.current_file;
        std::fs::File::create(current_path).unwrap();
    }
}

pub fn new_session(path: &str, path_exists: bool) -> Session {
    let new_db: MemDb = MemDb { path: "".to_string() };
    let session: Session = Session {
        path: path.to_string(),
        log_file: "".to_string(),
        current_file: "".to_string(),
        sst_path: "".to_string(),
        zkv: new_db,
    };
    let mut zkv_session = session.init();
    if !path_exists {
        println!("{}", path_exists);
        zkv_session.mk_files().unwrap();    
    }
    let db_file = zkv_session.sst_path.clone();
    zkv_session.zkv.path = db_file;
    zkv_session
}

pub fn create_files_list() -> std::io::Result<()> {
    utils::create_file_ticker(String::from("1"));
    Ok(())
}

pub fn init(path: &str) -> std::io::Result<Session> {
    let split_v: Vec<&str> = path.split("/").collect();
    if split_v.len() > 2 {
        panic!(format!("cannot init session here. {}", path))
    }

    let p = std::path::Path::new(path);
    let cur_db_exists = std::path::Path::exists(p);
    let session: Session = new_session(path, cur_db_exists);
    Ok(session)
}

#[cfg(test)]
pub mod test_session {
    use super::*;

    #[test]
    fn test_new_session() -> std::io::Result<()> {
        let db_path = std::string::String::from("./db");
        let sess = init(&db_path)?;
        let zkv_db_path = std::string::String::from("./db/DATA.sst");

        assert_eq!(sess.zkv.path, zkv_db_path);
        Ok(())
    }

    #[test]
    fn test_check_db_path() -> std::io::Result<()> {
        let db_path = std::string::String::from("./db");
        let _sess = init(&db_path)?;
        match std::fs::File::open("./db") {
            Ok(_) => assert!(true, "path exists"),
            Err(_) => assert!(false),
        };
        Ok(())
    }

    #[test]
    fn test_insert() -> std::io::Result<()> {
        let sess = init("./db")?;
        match sess.zkv.put("hello", "world") {
            Ok(_) => assert!(true, "good insert"),
            Err(_) => assert!(false, "bad insert"), 
        }
        assert_eq!(sess.zkv.get("hello"), "world");
        Ok(())
    }
}
