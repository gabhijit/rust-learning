//! REdis Serialization Protocol implementation in Rust
//!

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct RedisProtocolParser<'a> {
    input: &'a str, // Lifetime same as the passed input during `new` below
    index: usize,
}

// We are implementing this 'structure' to keep some internal state.
// Eventually when we'll support a buffered I/O for this, the internal
// state will change, but largely other code should remain as it is.
impl<'a> RedisProtocolParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, index: 0 }
    }

    pub fn advance(&mut self, howmuch: usize) -> () {
        self.index += howmuch;
    }

    pub fn consumed(&self) -> bool {
        self.index >= self.input.len()
    }

    pub fn parse(&mut self) -> Result<(), ValueError> {
        while !self.consumed() {
            let value = self.parse_one_value()?;
            println!("Parsed {:?}", value);
        }

        Ok(())
    }

    fn parse_one_value(&mut self) -> Result<Value, ValueError> {
        println!("Consumed: {:?}", self.consumed());

        let text_str = &self.input[self.index..];
        let first = text_str.as_bytes().get(0);
        match first {
            Some(b'*') => self.parse_array(),
            Some(b'$') => self.parse_string(),
            _ => {
                println!("First: {:?}", first);
                Err(ValueError::new("Not Supported!"))
            }
        }
    }

    fn parse_array(&mut self) -> Result<Value, ValueError> {
        let text_str = &self.input[self.index..];

        if let Some(idx) = text_str.find("\r\n") {
            println!("index1: {}", self.index);
            self.advance(idx + 2);
            println!("index2: {}", self.index);

            let length = text_str[1..idx].parse::<i32>()?;
            if length > 0 {
                let mut arr = Vec::new();
                for _ in 0..length {
                    if let Ok(value) = self.parse_one_value() {
                        arr.push(value);
                        println!("arr: {:?}, index: {:?}", arr, self.index);
                    } else {
                        println!("arr: {:?}, idx: {:?}", arr, idx);
                        return Err(ValueError::new("Unable to parse value"));
                    }
                }
                Ok(Value::Array(arr))
            } else {
                Ok(Value::NullArray)
            }
        } else {
            Err(ValueError::new("Unable to parse!"))
        }
    }

    fn parse_string(&mut self) -> Result<Value, ValueError> {
        let text_str = &self.input[self.index..];

        if let Some(idx) = text_str.find("\r\n") {
            println!("index3: {}", self.index);
            self.advance(idx + 2);
            println!("index4: {}", self.index);

            let length = text_str[1..idx].parse::<i32>()?;

            println!("{:?}", length);

            if length > 0 {
                let strval = String::from(&text_str[idx + 2..idx + 2 + length as usize]);

                println!("index5: {}", self.index);
                self.advance(length as usize + 2);
                println!("index6: {}", self.index);

                return Ok(Value::String(strval));
            } else {
                println!("index7: {}", self.index);
                self.advance(2);
                println!("index8: {}", self.index);
                return Ok(Value::NullString);
            }
        }

        Err(ValueError::new("Unable to Parse"))
    }
}

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
    cause: String,
}

// We need to define our own Error Type that can be used
// as a return type.
impl ValueError {
    fn new(cause: &str) -> ValueError {
        ValueError {
            cause: String::from(cause),
        }
    }
}

// This is required to be defined for our 'Error' Types
impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cause)
    }
}

// Implements our `ValueError` as an Error
impl Error for ValueError {}

// This allows us to use `?` Operator. The basic idea is as follows
impl From<std::num::ParseIntError> for ValueError {
    fn from(_: std::num::ParseIntError) -> ValueError {
        ValueError {
            cause: String::from("Unable to parse"),
        }
    }
}
