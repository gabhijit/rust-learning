/// An Implementation of Redis Protocol
use resp_rs;

fn main() {
    let _ = resp_rs::Value::from_resp("$hello\r\n$world\r\n").unwrap();
    /* match result {
        Ok(result) => result,
        Err(Error) => Err(Error),
    }; */
}