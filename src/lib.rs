pub mod db;
pub mod utils;
pub mod session;

pub mod test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test() {
        
    }

    #[test]
    fn check_db_path() {
        let f = std::fs::File::open("./db");
        match f {
            Err(_) => assert_eq!(1, 0),
            _ => assert_eq!(1, 1),
        }
    }
}
