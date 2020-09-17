extern crate zkv;

fn main() -> std::io::Result<()> {
    let db_path = std::env::args().nth(1).expect("'db' path is required");
    let db = match zkv::session::init(&db_path) {
        Ok(x) => x,
        Err(_) => panic!("bad"),
    };
    println!("{:#?}", db);
    let kv = db.zkv;
    let put_val = kv.put("hello", "world")?;
    println!("{:#?}", put_val);

    let get_val = kv.get("hello");
    println!("{:#?}", get_val);
    Ok(())
}
