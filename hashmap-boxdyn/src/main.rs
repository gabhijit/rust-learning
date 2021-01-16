use lazy_static::lazy_static;

use std::boxed::Box;
use std::collections::HashMap;
use std::sync::Mutex;

trait Foo {
    fn foo(&self) {}
    fn cloned(&self) -> Box<dyn Foo + Send + Sync>;
}

#[derive(Copy, Clone)]
struct S {}

impl Foo for S {
    fn cloned(&self) -> Box<dyn Foo + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Foo + Send + Sync> {
    fn clone(&self) -> Self {
        self.cloned()
    }
}

lazy_static! {
    static ref H: Mutex<HashMap<String, Box<dyn Foo + Send + Sync>>> = Mutex::new(HashMap::new());
}

fn get_foo(input: &str) -> Option<Box<dyn Foo + Send + Sync>> {
    {
        let map = H.lock().unwrap();

        let v = map.get(input).unwrap();
        let cloned = (*v).clone();
        Some(cloned)
    }
}

fn main() {
    {
        let mut map = H.lock().unwrap();
        map.insert("x".to_string(), Box::new(S {}));
    }

    if let Some(_) = get_foo("x") {
        println!("Some(x)");
    } else {
        println!("None");
    }
}
