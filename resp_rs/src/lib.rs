//! REdis Serialization Protocol implementation in Rust
//!

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum Value {
    Null,
    NullArray,
    NullString,
    Number(i64),
    String(String),
    Array(Vec<Value>),
}

#[derive(Debug)]
pub struct ValueError {
    cause: String
}

impl ValueError {
    fn new(cause: &str) -> ValueError {
        ValueError { cause: String::from(cause) }
    }
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cause)
    }
}

impl Error for ValueError {}

impl From<std::num::ParseIntError> for ValueError {
    fn from(_: std::num::ParseIntError) -> ValueError {
        ValueError { cause: String::from("Unable to parse") }
    }
}

impl Value {
    pub fn from_resp(input: &str) -> Result<Value, ValueError> {

        let first = input.as_bytes().get(0);
        match first {
            Some(b'*') => { Value::parse_list(input) },
            Some(b'$') => { Value::parse_string(input) },
                _ => { Err(ValueError::new("Not Supported!")) }
        }
            /*
            if let Some(idx) = input[current..].find("\r\n") {
                println!("{:?}, {:?}, {:?}", idx, current, input.len());
                if let Ok(x) = Value::parse_item(&input[current..current + idx]) {
                    current = current + idx + 2;
                    if current > input.len() {
                        break;
                    }
                }
            } else {
                break;
            }
            */
    }

    fn parse_list(input: &str) -> Result<Value, ValueError> {

        if let Some(idx) = input.find("\r\n") {
            let length = input[1..idx].parse::<i32>()?;
            println!("{:?}", length);
            let mut arr = Vec::new();
            for _ in 0..length {
                if let Ok(value) = Value::from_resp(&input[idx + 2..]) {
                    arr.push(value);
                } else {
                    return Err(ValueError::new("Unable to parse value"));
                }
            }
            Ok(Value::Array(arr))
        } else {
            Err(ValueError::new("Unable to parse!"))
        }
    }

    fn parse_string(input: &str) -> Result<Value, ValueError> {

        if let Some(idx) = input.find("\r\n") {
            let length = input[1..idx].parse::<i32>()?;
            println!("{:?}", length);
            if length > 0 {
                let strval = String::from(&input[idx + 2 .. idx + 2 + length as usize]);
                return Ok(Value::String(strval));
            } else {
                return Ok(Value::NullString);
            }
        }

        Err(ValueError::new("Unable to Parse"))

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
