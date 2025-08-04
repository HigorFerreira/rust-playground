use std::fs;
use std::io;

fn main() {
    let username: Option<String> = match read_username_from_file() {
        Ok(r) => Some(r),
        Err(_) => None
    };

    println!("{}", match username {
        Some(str) => str,
        None => String::from("")
    })
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}