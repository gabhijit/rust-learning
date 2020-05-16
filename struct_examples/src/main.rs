// Struct Examples


#[derive(Debug)]
struct User {
    name: String,
    email: String,
    active: bool,
    num_logins: u32

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }

    // It's quite possible to pass
    // &mut self -> When you want to change something in the self or
    // self -> When you want to consume self. Usually &self, if we
    // simply want to read and do stuff with that.
    // Note: `self` is also a keyword and has special meaning, so
    // you cannot have bound methods unless self is
    fn area(&self)  -> u32 {
        self.width * self.height
    }

}

fn main() {
    let u = User {
        name: String::from("Someone"),
        email: String::from("someone@example.com"),
        active: true,
        num_logins: 1
    };
    // derive(Debug) above allows one to use
    // {:?} : Normal Debug Format and {:#?} : Pretty Print format.
    // However, if you really have to implement {}, it needs a
    // `display::Format` to be implemented for the struct.
    //
    println!("Hello, {:#?}", u);

    let size = 10;
    let sq  = Rectangle::square(size);
    println!("Square of size {} from {:?} has area {}", size, sq, sq.area());
}
