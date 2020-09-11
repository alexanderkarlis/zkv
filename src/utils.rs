pub fn create_file_ticker(current: std::string::String) -> std::string::String {
    let mut num: i32 = current.parse().unwrap();
    num += 1;
    format!("{:#?}", num)
}
