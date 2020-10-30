#![allow(dead_code)]


#[derive(Debug)]
// This means borrowed reference should outlive the struct S
struct S<'a> {
    // Following is not allowed.
    // x: &u32
    // instead does not work as well if we use a local variable.
    //x: &'static u32
    x: &'a u32

}


// Let's write a simple function that returns reference to an
// internal borrowed value for the above structure
// First start with no life-time definitions
// Clearly somewhere the lifetime needs to be specified.
//fn largest_x(first: &S, second: &S) -> & u32 {

// Following will work because everyone gets the same lifetime
//
fn largest_x<'a>(first: &'a S, second: &'a S) -> &'a u32
{
// fn largest_x<'a>(first: &S, second: &S) -> &'a u32 {

    if first.x > second.x {
        first.x
    } else {
        second.x
    }
}


fn print_val(x: & str) {
    println!("{:?}", x);
}

// static SS: S = S{x: &100};

fn main() {


    let f: S;

    {
        let x: u32 = 100;
        f = S { x: &x};

        // If the following println! is moved out of block, since x goes out of scope,
        // we get an error, so borrowed value should ideally outlive the struct.
        println!("{:?}", f);
    }

}
