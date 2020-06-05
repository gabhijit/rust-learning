
use super::*;

#[test]
fn test_emptry_array_success() {

    let result = parser::RedisProtocolParser::new(b"*0\r\n").parse();

    let empty = Vec::new();

    assert_eq!( Ok(value::Value::Array(empty)), result);
}

#[test]
fn test_emptry_array_failure() {

    let result = parser::RedisProtocolParser::new(b"*0").parse();

    assert_eq!( result.is_err(), true);
}

#[test]
fn test_emptry_array_extra_chars() {

    let result = parser::RedisProtocolParser::new(b"*0\r\nfoo").parse();

    assert_eq!( result.is_err(), true);
}

