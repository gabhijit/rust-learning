//! REdis Serialization Protocol implementation in Rust
//!

use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub enum Value {
    Null,
    NullArray,
    Number(i64),
    String(String),
}

impl Value {

    pub fn from_resp(input: &str) -> Result<Value, std::io::Error> {

        for item in input.split("\r\n") {
            let last = Value::parse_item(item);
            match last {
                Err(_) => println!("None"),
                Ok(x) => println!("{:?}", x),
            }
        }
        Ok(Value::Null)
    }

    fn parse_item(item: &str) -> Result<Value, std::io::Error> {
        match  item {
            "$-1" => Ok(Value::Null),
            "*-1" => Ok(Value::NullArray),
            _  => Ok(Value::String(String::from(item))),
        }
    }

}
