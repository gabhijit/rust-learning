//! REdis Serialization Protocol implementation in Rust
//!

use std::io::{Error, ErrorKind};

pub enum Value {
    Null,
    Number,
}

impl Value {

    pub fn from_resp(input: &str) -> Result<Value, std::io::Error> {
        if input.len() > 0 {
            Ok(Value::Null)
        } else {
            Err(Error::new(ErrorKind::Other, "oh no!"))
        }
    }

}
