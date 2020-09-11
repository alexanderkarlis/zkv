extern crate zkv;

fn main() {
    let db_path = std::env::args().nth(1).expect("'db' path is required");
    // let db: zkv::zkv::Db;
    let db = match zkv::session::init(&db_path) {
        Ok(x) => x,
        Err(_) => panic!("bad"),
    };
    println!("{:#?}", db);
}
