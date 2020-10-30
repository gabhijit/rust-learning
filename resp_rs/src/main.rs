/// An Implementation of Redis Protocol

fn main() {
    let mut parser = resp_rs::parser::RedisProtocolParser::new(
        b"*5\r\n*1\r\n:10\r\n$5\r\nhello\r\n$-1\r\n:42\r\n+Something\r\n",
    );

    let result = parser.parse().unwrap();
    println!("{:?}", result);

    let result = resp_rs::parser::RedisProtocolParser::new(b"*0\r\n")
        .parse()
        .unwrap();
    println!("{:?}", result);
}
