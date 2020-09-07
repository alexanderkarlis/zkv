use crate::utils;
use String;

#[derive(Debug)]
pub struct Db {
    // will i need <'a> ??
    pub path: String,
    log_file: String,
    lock_file: String,
    current_file: String,
    current_bkp: String,
}

impl Db {
    /// # Create Db Files
    /// *This function is designed create the necessary files*
    /// *in order to adequately start the zkv*
    ///
    /// * LOG => logs all transactions to/from db
    /// * LOCK  => not sure yet...?
    /// * CURRENT - Text  => current manifest to read
    /// * CURRENT.bak  => backup manifest to read from
    pub fn init(mut self) -> Db {
        let mut p_log = self.path.clone();
        let mut p_lock = self.path.clone();
        let mut p_current = self.path.clone();
        let mut p_bak = self.path.clone();
        p_log.push_str("/LOG");
        p_lock.push_str("/LOCK");
        p_current.push_str("/CURRENT");
        p_bak.push_str("/CURRENT.bak");
        self.log_file = p_log;
        self.lock_file = p_lock;
        self.current_file = p_current;
        self.current_bkp = p_bak;
        self.mk_files().unwrap();
        self
        
    }

    pub fn mk_files(&self) -> std::io::Result<()> {
        self.create_log_file();
        self.create_lock_file();
        self.create_current_file();
        self.create_bak_file();
        Ok(())
    }

    /// take in an extra param to do incrementation
    fn create_log_file(&self) {
        let log_path: &str = &self.log_file;
        std::fs::File::create(log_path).unwrap();
    }

    /// take in an extra param to do incrementation
    fn create_lock_file(&self) {
        let lock_path: &str = &self.lock_file;
        std::fs::File::create(lock_path).unwrap();
    }

    /// take in an extra param to do incrementation
    fn create_current_file(&self) {
        let current_path: &str = &self.current_file;
        std::fs::File::create(current_path).unwrap();
    }
    /// take in an extra param to do incrementation
    fn create_bak_file(&self) {
        let bak_path: &str = &self.current_bkp;
        std::fs::File::create(bak_path).unwrap();
    }
}

pub fn new_db(path: &str) -> Db {
    let db: Db = Db {
        path: path.to_string(),
        log_file: "".to_string(),
        lock_file: "".to_string(),
        current_file: "".to_string(),
        current_bkp: "".to_string(),
    };
    let zkv = db.init();
    zkv
}

pub fn create_files_list() -> std::io::Result<()> {
    utils::create_file_ticker(String::from("1"));
    Ok(())
}

pub fn init(path: &str) -> std::io::Result<Db> {
    let split_v: Vec<&str> = path.split("/").collect();
    if split_v.len() > 2 {
        panic!(format!("cannot init db here. {}", path))
    }

    // TODO: check and see if path alrady exists otherwise return files and stuff
    std::fs::create_dir_all(path)?;
    let db: Db = new_db(path);
    Ok(db)
}