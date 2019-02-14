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

        let mut current = 0;
        let curr_item = Value::Null;
        let mut item = "";

        loop {
            let idx = input[current..].find("\r\n");

            match idx {
                Some(x) =>  {
                    item = &input[current..current+x];
                    current += x + 2;
                    Value::parse_item(item);
                },
                None => {
                    println!("None");
                    Error::new(ErrorKind::UnexpectedEof, "String must end with '\r\n'");
                },
            }
        }
    }

    fn parse_item(item: &str) -> Result<Value, std::io::Error> {
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
