use std::error::Error;
use std::fmt;
use std::str;

#[derive(Debug)]
pub enum Value {
    SimpleString(String),
    Okay,
    Error(String),
    Integer(i64),
    BulkString(Vec<u8>),
    NullBulkString,       // Special case of BulkString "$-1\r\n"
    Array(Vec<Value>),
    NullArray             // Special case of Array "*-1\r\n"
}

#[derive(Debug)]
pub struct ValueError {
    cause: String,
}

// We need to define our own Error Type that can be used
// as a return type.
impl ValueError {
    pub(crate) fn new(cause: &str) -> ValueError {
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

impl From<str::Utf8Error> for ValueError {

    fn from(_: str::Utf8Error) -> ValueError {
        ValueError {
            cause: String::from("Unable to parse Utf8 String"),
        }
    }
}
