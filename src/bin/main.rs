extern crate zkv;

fn main() {
    let db_path = std::env::args().nth(1).expect("no pattern given");
    // let db: zkv::zkv::Db;
    let db = match zkv::db::init(&db_path) {
        Ok(x) => x,
        Err(_) => panic!("bad"),
    };
    println!("{:#?}", db);
}
