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

        let mut last = Value::Null;

        for item in input.split("\r\n") {
            let result = Value::parse_item(item);
            match result {
                Err(err) =>  {
                    println!("None");
                    return Err(err)
                },
                Ok(x) => {
                    println!("{:?}", x);
                    match x {
                        Value::Null => println!("Null"),
                        Value::NullArray => println!("NullArray"),
                        _ => println!("Neither"),
                    }

                    last = x
                },
            }
        }
        Ok(last)
    }

    fn parse_item(item: &str) -> Result<Value, std::io::Error> {
        match item {
            "$-1" => Ok(Value::Null),
            "*-1" => Ok(Value::NullArray),
            _  => Ok(Value::String(String::from(item))),
        }
    }

}
