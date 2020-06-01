//! REdis Serialization Protocol implementation in Rust
//!


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

        loop {
            if let Some(idx) = input[current..].find("\r\n") {
                println!("{:?}, {:?}, {:?}", idx, current, input.len());
                if let Ok(x) = Value::parse_item(&input[current..current+idx]) {
                    current = current + idx + 2;
                    if current > input.len() {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        Ok(Value::Null)
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
