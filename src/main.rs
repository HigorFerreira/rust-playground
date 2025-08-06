use std::{net::IpAddr, str::FromStr};

fn main() {
    let home: Option<IpAddr> = match "127.0.0.1".parse() {
        Ok(addr) => Some(addr),
        _ => None
    };

    println!("{home:?}");
}