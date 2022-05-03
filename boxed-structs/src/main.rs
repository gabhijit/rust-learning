#[derive(Debug)]
struct S {
    v: Vec<Box<i32>>,
}

fn main() {
    let s = S {
        v: vec![Box::new(1_i32)],
    };

    // s.last().unwrap() returns a reference to the last element in this case Box. so s.v.last() is
    // a &Box<i32>
    let mut last = s.v.last().unwrap();

    let second_last = &Box::new(2_i32);

    last = second_last;

    println!("Hello, world!, s: {:#?}, last: {:#?}", s, last);
}
