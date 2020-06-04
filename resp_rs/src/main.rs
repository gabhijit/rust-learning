/// An Implementation of Redis Protocol
use resp_rs;

fn main() {

    let mut parser = resp_rs::RedisProtocolParser::new(b"*4\r\n*1\r\n:10\r\n$5\r\nhello\r\n$-1\r\n:42\r\n");

    let result = parser.parse().unwrap();
    println!("{:?}", result);
    //let result = resp_rs::Value::from_resp("$-1\r\n").unwrap();
    //println!("{:?}", result);
    //println!("Done!");
    /* match result {
        Ok(result) => result,
        Err(Error) => Err(Error),
    }; */
}
