use crate::utils;
use String;

#[derive(Debug)]
pub struct Session {
    pub path: String,
    log_file: String,
    lock_file: String,
    current_file: String,
}

impl Session {
    /// # Create Session Files
    /// *This function is designed create the necessary files*
    /// *in order to adequately start the zkv*
    ///
    /// * LOG => logs all transactions to/from session
    /// * LOCK  => not sure yet...?
    /// * CURRENT - Text  => current manifest to read
    pub fn init(mut self) -> Session {
        let mut p_log = self.path.clone();
        let mut p_lock = self.path.clone();
        let mut p_current = self.path.clone();
        // is there a better way to do this??
        p_log.push_str("/LOG");
        p_lock.push_str("/LOCK");
        p_current.push_str("/CURRENT");
        self.log_file = p_log;
        self.lock_file = p_lock;
        self.current_file = p_current;
        self.mk_files().unwrap();
        self
    }

    pub fn mk_files(&self) -> std::io::Result<()> {
        self.create_log_file();
        self.create_lock_file();
        self.create_current_file();
        Ok(())
    }

    fn create_log_file(&self) {
        let log_path: &str = &self.log_file;
        std::fs::File::create(log_path).unwrap();
    }

    fn create_lock_file(&self) {
        let lock_path: &str = &self.lock_file;
        std::fs::File::create(lock_path).unwrap();
    }

    /// take in an extra param to do incrementation
    fn create_current_file(&self) {
        let current_path: &str = &self.current_file;
        std::fs::File::create(current_path).unwrap();
    }
}

pub fn new_session(path: &str) -> Session {
    let session: Session = Session {
        path: path.to_string(),
        log_file: "".to_string(),
        lock_file: "".to_string(),
        current_file: "".to_string(),
    };
    let zkv_session = session.init();
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

    // TODO: check and see if path alrady exists otherwise return files and
    // other cool stuff
    std::fs::create_dir_all(path)?;
    let session: Session = new_session(path);
    Ok(session)
}
