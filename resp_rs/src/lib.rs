//! REdis Serialization Protocol implementation in Rust
//!

use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub enum Value {
    Null,
    NullArray,
    Number(i64),
    String(String),
    Array(Vec<Value>),
}

impl Value {

    pub fn from_resp(input: &str) -> Result<Value, std::io::Error> {

        let items: Vec<_> = input.split("\r\n").collect();

        println!("{:?}" , items);
        for item in items.iter() {

            println!("Item: {}", item);
        }
        Ok(Value::Null)
    }

    fn parse_items(item: &Vec<str>) -> Result<Value, std::io::Error> {
        let first = item.as_bytes().get(0);
        match first {
            Some(b'*') => println!("*"),
            Some(b'$') => println!("$"),
            Some(b':') => println!(":"),
            Some(b'+') => println!("+"),
            Some(b'-') => println!("-"),
            None => println!("None"),
            _ => println!("Nothing"),
        }
        Ok(Value::Null)
    }

}
