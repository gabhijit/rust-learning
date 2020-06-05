use std::str;

use crate::value::{Value, ValueError};

#[derive(Debug)]
pub struct RedisProtocolParser<'a> {
    input: &'a [u8], // Lifetime same as the passed input during `new` below
    index: usize,
}

// We are implementing this 'structure' to keep some internal state.
// Eventually when we'll support a buffered I/O for this, the internal
// state will change, but largely other code should remain as it is.
impl<'a> RedisProtocolParser<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self { input, index: 0 }
    }

    pub fn advance(&mut self, howmuch: usize) -> () {
        self.index += howmuch;
    }

    pub fn consumed(&self) -> bool {
        self.index >= self.input.len()
    }

    pub fn parse(&mut self) -> Result<Value, ValueError> {
        let mut value = Value::NullBulkString;
        while !self.consumed() {
            value = self.parse_one_value()?;
        }

        Ok(value)
    }

    fn find_crlf(heystack: &[u8]) -> Option<usize> {
        let needle = &b"\r\n"[..];

        heystack.windows(needle.len()).position(|x| x == needle)
    }

    fn parse_one_value(&mut self) -> Result<Value, ValueError> {
        let text_str = &self.input[self.index..];
        let first = text_str.get(0);
        match first {
            Some(b'*') => self.parse_array(),
            Some(b'$') => self.parse_bulkstr(),
            Some(b':') => self.parse_integer(),
            Some(b'+') => self.parse_simplestr(),
            Some(b'-') => self.parse_errorstr(),
            _ => {
                println!("First: {:?}", first);
                Err(ValueError::new("Not Supported!"))
            }
        }
    }

    fn parse_array(&mut self) -> Result<Value, ValueError> {
        let text_str = &self.input[self.index..];

        if let Some(idx) = RedisProtocolParser::find_crlf(text_str) {
            self.advance(idx + 2);

            let s = str::from_utf8(&text_str[1..idx])?;
            let length = s.parse::<i32>()?;
            if length >= 0 {
                let mut arr = Vec::new();
                for _ in 0..length {
                    if let Ok(value) = self.parse_one_value() {
                        arr.push(value);
                    } else {
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

    fn parse_bulkstr(&mut self) -> Result<Value, ValueError> {
        let text_str = &self.input[self.index..];

        if let Some(idx) = RedisProtocolParser::find_crlf(text_str) {
            self.advance(idx + 2);

            let s = str::from_utf8(&text_str[1..idx])?;
            let length = s.parse::<i32>()?;

            if length > 0 {
                let strval = text_str[idx + 2..idx + 2 + length as usize].to_vec();

                self.advance(length as usize + 2);

                return Ok(Value::BulkString(strval));
            } else {
                return Ok(Value::NullBulkString);
            }
        }

        Err(ValueError::new("Unable to Parse"))
    }

    fn parse_integer(&mut self) -> Result<Value, ValueError> {
        let text_str = &self.input[self.index..];

        if let Some(idx) = RedisProtocolParser::find_crlf(text_str) {
            self.advance(idx + 2);

            let s = str::from_utf8(&text_str[1..idx])?;
            let int = s.parse::<i64>()?;

            return Ok(Value::Integer(int));
        }

        Err(ValueError::new("Unable to Parse"))
    }

    fn parse_simplestr(&mut self) -> Result<Value, ValueError> {
        let text_str = &self.input[self.index..];

        if let Some(idx) = RedisProtocolParser::find_crlf(text_str) {
            self.advance(idx + 2);

            let s = str::from_utf8(&text_str[1..idx])?;

            if s.starts_with("OK") {
                return Ok(Value::Okay);
            } else {
                return Ok(Value::SimpleString(String::from(s)));
            }
        }

        Err(ValueError::new("Unable to Parse"))
    }

    fn parse_errorstr(&mut self) -> Result<Value, ValueError> {
        let text_str = &self.input[self.index..];

        if let Some(idx) = RedisProtocolParser::find_crlf(text_str) {
            self.advance(idx + 2);

            let s = str::from_utf8(&text_str[1..idx])?;

            return Ok(Value::Error(String::from(s)));
        }

        Err(ValueError::new("Unable to Parse"))
    }
}
