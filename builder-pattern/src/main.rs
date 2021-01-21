use std::convert::TryFrom;
use std::error::Error as StdError;

#[derive(Debug, Copy, Clone)]
struct Error(usize);

impl StdError for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Bar {
    value: u8,
}

impl TryFrom<u8> for Bar {
    type Error = Error;

    fn try_from(x: u8) -> Result<Self, Self::Error> {
        Ok(Bar { value: x })
    }
}

impl TryFrom<&str> for Bar {
    type Error = Error;

    fn try_from(x: &str) -> Result<Self, Self::Error> {
        let x = x.parse::<u8>().map_err(|_| Error(42))?;
        Ok(Bar { value: x })
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Foo {
    bar: Bar,
}

impl Foo {
    fn builder() -> Builder {
        Builder::new()
    }
}

#[derive(Debug, Copy, Clone)]
struct Builder {
    inner: Result<Foo, Error>,
}

impl Builder {
    fn new() -> Self {
        Builder {
            inner: Ok(Foo::default()),
        }
    }

    fn bar<T>(self, bar: T) -> Self
    where
        Bar: TryFrom<T>,
        <Bar as TryFrom<T>>::Error: Into<Error>,
    {
        self.and_then(move |mut x| {
            println!("x: {:?}", x);
            x.bar = TryFrom::try_from(bar).map_err(Into::into)?;
            Ok(x)
            //uOk(Foo::default())
        })
    }

    fn build(self) -> Result<Foo, Error> {
        self.inner
    }

    fn and_then<F>(self, f: F) -> Self
    where
        F: FnOnce(Foo) -> Result<Foo, Error>,
    {
        Builder {
            inner: self.inner.and_then(f),
        }
    }
}

fn main() {
    println!("Foo {:?}", Foo::builder().build().map_err(|_| "Hello"));

    let foo = Foo::builder();
    println!(
        "Foo {:?}",
        foo.bar(42).bar("84").build().map_err(|_| "Hello")
    );
    println!("Foo {:?}", foo.bar("22").build().map_err(|_| "Hello"));
    println!(
        "Foo {:?}",
        Foo::builder().bar("bar").build().map_err(|_| "Hello")
    );
}
