use std::convert::{TryFrom, TryInto};

#[derive(Debug, Default)]
struct Bar {
    value: u8,
}

impl TryFrom<u8> for Bar {
    type Error = std::io::Error;

    fn try_from(x: u8) -> Result<Self, Self::Error> {
        Ok(Bar { value: x })
    }
}

impl TryFrom<&str> for Bar {
    type Error = std::io::Error;

    fn try_from(x: &str) -> Result<Self, Self::Error> {
        let x = x
            .parse::<u8>()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("{}", x)))?;
        Ok(Bar { value: x })
    }
}

#[derive(Debug, Default)]
struct Foo {
    bar: Bar,
}

impl Foo {
    fn builder() -> Builder {
        Builder::new()
    }
}

#[derive(Debug)]
struct Builder {
    inner: Result<Foo, std::io::Error>,
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
        <Bar as TryFrom<T>>::Error: Into<std::io::Error>,
    {
        self.and_then(move |mut x| {
            println!("x: {:?}", x);
            x.bar = TryFrom::try_from(bar).map_err(Into::into)?;
            Ok(x)
            //uOk(Foo::default())
        })
    }

    fn build(self) -> Result<Foo, std::io::Error> {
        self.inner
    }

    fn and_then<F>(self, f: F) -> Self
    where
        F: FnOnce(Foo) -> Result<Foo, std::io::Error>,
    {
        Builder {
            inner: self.inner.and_then(f),
        }
    }
}

fn main() {
    println!("Foo {:?}", Foo::builder().build().map_err(|_| "Hello"));
    println!(
        "Foo {:?}",
        Foo::builder().bar(42).build().map_err(|_| "Hello")
    );
    println!(
        "Foo {:?}",
        Foo::builder().bar("22").build().map_err(|_| "Hello")
    );
    println!(
        "Foo {:?}",
        Foo::builder().bar("bar").build().map_err(|_| "Hello")
    );
}
