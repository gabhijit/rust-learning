struct Struct {}

trait Trait {
    type Value;

    fn do_something(&self, value: &Self::Value);
}

impl Trait for Struct {
    type Value = (i64, String);

    fn do_something(&self, (a, b): &(i64, String)) {
        println!("{} {}", a, b);
    }
}

fn main() {
    let a = 10;
    let b = "foo".to_owned();
    let s = Struct {};

    let _t = (a, b.clone());
    s.do_something(&(a, b.clone())); // This works...
    println!("{}", b); // ...but the borrow checker complains here
}
