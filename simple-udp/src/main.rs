use std::net::UdpSocket;
use std::{thread, time};

static COUNTER: u32 = 0;

fn main() -> std::io::Result<()> {
    let addr = "0.0.0.0:4739";
    let socket = UdpSocket::bind(addr)?;
    println!("listening on: {}", addr);
    let mut counter = 0;
    thread::spawn(|| loop {
        println!("thread counter: {}", &counter);

        let ten_millis = time::Duration::from_secs(10);
        thread::sleep(ten_millis);
    });

    let mut buf = [0; 64 * 1024];
    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        counter += 1;
        println!("n: {}, src: {}, counter: {}", amt, src, counter);
    }

    Ok(())
}
