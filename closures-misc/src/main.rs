fn adder(y: usize) -> impl Fn(usize) -> usize {
    move |x| y + x
}

#[derive(Debug)]
pub struct Wrapper<T> {
    pub elem: T,
}

fn main() {
    let wrapper: Wrapper<_> = Wrapper { elem: adder(5) };
}
