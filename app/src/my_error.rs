#[derive(Debug)]
pub struct MyError {
    pub msg: String,
}
impl MyError {
    pub fn new_str(s: &str) -> MyError {
        MyError { msg: s.to_string() }
    }
    pub fn new_string(s: String) -> MyError {
        MyError { msg: s }
    }
}
