use std::{fs::File, io::{ErrorKind, Write}};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(mut fc) => {
                    let content = "Hello, world!".as_bytes();
                    match fc.write_all(content) {
                        Ok(_) => fc,
                        Err(e) => panic!("Problem while writing to hello.txt: {e:?}")
                    }
                },
                Err(e) =>  panic!("Problem creating the file: {e:?}")
            },
            _ => {
                panic!("Problem oppening the file {error:?}")
            }
        }
    };

    println!("{:?}", greeting_file);
}