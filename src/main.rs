use std::{fs::File, io::{ErrorKind, Write}};

fn main() {
    
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            match File::create("hello.txt") {
                Err(error) => panic!("Problem creating the file: {error:?}"),
                Ok(mut fc) => {
                    if let Err(error) = fc.write_all("Hello, world!".as_bytes()) {
                        panic!("Problem while writing to hello.txt: {error:?}")
                    } else {
                        fc
                    }
                }
            }
        } else {
            panic!("Problem oppening the file {error:?}")
        }
    });

    println!("{:?}", greeting_file);
}