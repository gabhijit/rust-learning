// playing around with Enums

#[derive(Debug)]
struct IPv4Addr(u8, u8, u8, u8);

#[derive(Debug)]
struct IPv6Addr;

#[derive(Debug)]
enum IPAddr {
    V4(IPv4Addr),
    V6(IPv6Addr)
}

#[derive(Debug)]
enum Message {
    QuitMessage,
    ChangeColor(u32, u32, u32),
    Move { x: i32, y: i32},
    Write(String)
}





fn main() {

    let ip = IPAddr::V4(IPv4Addr(127,0,0,1));

    println!("This is {:?}", ip);

    let message = Message::QuitMessage;
    println!("message is {:?}.", message);

    let color = Message::ChangeColor(1, 0, 2);

    // Basically the `if let below` is  equivalent t-
    //
    if let Message::ChangeColor(x, y, z) = color {
        println!("Change Color requested to {}, {}, {}", x, y, z);
    } else {
        println!("message is {:?}", color);
    }

    match color {
        Message::ChangeColor(_, _, _) => println!("Matched"),
        _ => println!("No match!")
    }

    // Following works - but if we use anything that uses `move`
    // We'll get a compile time error! That is like Wow!
    // This is because `unwrap` consumes the value!
    // So once the value is returned by `unwrap`, you cannot borrow
    // again. The following works 'cos built-in types are copieed
    let x = Some(2);
    println!("Contained value {} {}", x.unwrap(), x.unwrap());
    println!("x is {:?}", x);

    let x = Some(String::from("hello"));
    println!("Contained value {}", x.unwrap());
    //println!("x is {:?}", x);
}
