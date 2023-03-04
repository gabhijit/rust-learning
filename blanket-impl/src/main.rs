trait Animal {
    fn speaks(&self) -> String;
}

// One will often come across the code like following and it is pretty
// difficult to 'understand' what is happening below implement a Trait for
// something that is a reference to the same Trait is like 'WTF' :-)
//
// This is really a blanket implementation for all 'Animal's whose Reference
// we have, and we are really calling the original `speaks` method of the
// Animal whose reference we have. And since we have a reference, we need to
// pass the lifetimes without which the following code won't compile.
impl<'a, A> Animal for &'a A
where
    A: Animal,
{
    fn speaks(&self) -> String {
        (**self).speaks()
    }
}

impl<A> Animal for Box<A>
where
    A: Animal,
{
    fn speaks(&self) -> String {
        (**self).speaks()
    }
}

#[derive(Debug)]
struct Cat {
    name: String,
}

#[derive(Debug)]
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn speaks(&self) -> String {
        String::from("Bhow Bhow!")
    }
}

#[derive(Debug)]
struct Man<T>
where
    T: Animal,
{
    pet: Option<T>,
}

impl<T: Animal> Man<T> {
    fn new(_pet: Option<T>) -> Self {
        Self { pet: _pet }
    }
}

impl<T: Animal> Animal for Man<&T> {
    fn speaks(&self) -> String {
        String::from("Hellos!")
    }
}

fn main() {
    let maybe_dogowner: Man<&Dog> = Man::new(None);

    let bruno = Dog {
        name: "Bruno".to_string(),
    };

    let milo = Dog {
        name: "Milo".to_string(),
    };

    let shira = Box::new(Dog {
        name: "Shira".to_string(),
    });

    // It is okay to do Some(value) as well as Some(reference) because
    // Trait Animal is implemented for for both The value Dog and
    // Reference to Dog through Blanket implementation above!
    let doglover = Man::new(Some(bruno));

    let dogtrainer = Man::new(Some(&milo));

    let boxed_dog_refowner = Man::new(Some(&shira));

    let boxed_dogowner = Man::new(Some(&shira));

    println!("{:?}", maybe_dogowner);
    println!("{:?}", doglover);
    println!("{:?}", dogtrainer);
    println!("{:?}", boxed_dog_refowner);
    println!("{:?}", boxed_dogowner);
}
