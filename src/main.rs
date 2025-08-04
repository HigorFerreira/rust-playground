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
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;
    
    Ok(username)
}