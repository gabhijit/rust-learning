use std::any::Any;

trait Foo {
    fn foo(&self) -> ();

    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
struct S<'a> {
    value: &'a str,
}

impl<'a> Foo for S<'a> {
    fn foo(&self) -> () {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let s = "Hello World!";
    let ss = S { value: s };
    println!("{:?}", ss);
}
