/// An Implementation of Redis Protocol
use resp_rs;

fn main() {
    let result = resp_rs::Value::from_resp("$5\r\nhello\r\n").unwrap();
    println!("{:?}", result);
    let result = resp_rs::Value::from_resp("$-1\r\n").unwrap();
    println!("{:?}", result);
    println!("Done!");
    /* match result {
        Ok(result) => result,
        Err(Error) => Err(Error),
    }; */
}
