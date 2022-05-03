#[derive(Debug)]
pub struct MyRefCell<T> {
    data: T,
}

impl<T> MyRefCell<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }

    pub fn borrow_mut(&self) -> &mut T {
        unsafe { &mut *(&self.data as *const T as *mut T) }
    }
}

fn main() {
    let rc = MyRefCell::new(0);
    println!("{:?}", rc);
    let b1 = rc.borrow_mut();
    let b2 = rc.borrow_mut();
    *b1 += 1;
    *b2 += 1;
    println!("{:?}", rc);
    *b1 += 1;
    *b2 += 1;
    println!("{:?}", rc);
}
