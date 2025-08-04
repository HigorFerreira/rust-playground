use std::{fs::File, io::{self, ErrorKind, Read, Write}};

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
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}