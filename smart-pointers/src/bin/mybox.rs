// Defining 'MyBox' type, like a Box


use std::ops::Deref;

// Following 'type' is not a good idea, may be there is a way to make it work,
// but I don't know at the moment
//type MyBox<T> (T);

// instead -- use derive(Debug) to be able to print.
#[derive(Debug)]
struct MyBox<T> (T);

// If we simply
// impl MyBox<T> {
// Above `T` is not found in this scope understandably. Is there a trickery to bring it in?
// But following works
impl<T> MyBox<T> {

    fn new(t:T) -> Self {
        // Simply returning
        // (t) - Won't Make sense due to `Self` above
        // So we've to make a 'MyBox' out of it.
        MyBox(t)
    }

}

impl<T> Deref for MyBox<T> {

    // If we do not give this type, Compilation fails.
    // Because `Deref`'s signature specifies return type of `deref` as `Self::Target`
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }

}

fn main() {

    let x = MyBox::new(5);

    // Following is same as  `*(x.deref())`;
    // And since we have now Deref implemented we are good.
    println!("{:?}", *x);
}
