// Trying to implement MySum as a Sum Trait. Just to get a  better understanding of the syntax
// This is a totally weird implementation of a trait, but that has helped me understand a few
// things. Let us look at the definition of the following trait (a completely convoluted one, but
// helps one understand things, so not all that bad.)
// First <A = Self> is a default parameter - So yes MySum is a Generic Trait. (And everything below
// is actually taken from std::iter::Sum) Where you can take a Generic Parameter <A>, if none is
// specified, it's the Self. Which is given in the shortcut.
//
// So one can simply `impl MySum for f32` say. No need to say `MySum<f32> for <f32>` because `f32`
// will be assumed to be the type (A = Self)
//
trait MySum<A = Self> {
    fn sum<I: Iterator<Item = A>>(iter: I) -> Self;
}

// Since now I am doing a 'concrete implementation', I will have to use
impl MySum<i32> for f32 {
    // If I try just the following - Nothing wrong, but since the type of the 'Item' is not known,
    // I am not allowed to use simply `x as f32` below.
    //fn sum<I: Iterator>(iter: I) -> Self {

    // So Instead Following works - Type of `Item` specified.
    fn sum<I: Iterator<Item = i32>>(iter: I) -> Self {
        let sum = iter.fold(0.0, |sum, x| sum + x as f32);
        sum
    }
}

// A couple of interesting points below, as long as T implements `Copy` trait, we are good. If we
// remove the `+ Copy` part, we'll be required to get the `reference to list[0], but if T
// implements 'Copy' trait, the value returned will be a `Copy` of the element and not a reference
// to the element and hence will be allowed. Also, we should be returning the `Copy` and not the
// `reference`.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers: Vec<i32> = vec![134, 50, 25, 100, 65];

    // If I uncomment the following, I am not allowed to do so because, once I borrow, I am not
    // allowed to `move out` below in the `into_iter`.
    // This cannot be fixed with the lifetimes, Reason: if we returned a `reference` to largest
    // element, subsequent `MySum::sum` , which 'consumes' the vector, that means reference to
    // the `largest` element is invalid.
    let largest = largest(&numbers);

    // `into_iter()` is important one, if we simply use `iter()`, it iterates over references that
    // is `&i32`. Now the problem with that is. This will require to change the implementation
    // above to use `iter.fold(0.0, |sum, &x| sum + x as f32 );`, but this will not agree with the
    // definition of `fold` method of the iterator, because `&x` cribs, we need iterator over
    // `elements` and not `reference to elements` and that is provided by `into_iter` so we simply
    // use that.
    let sum: f32 = MySum::sum(numbers.into_iter());

    println!("The sum is {:?}", sum);
    println!("The sum is {:?}, largest number is {:?}", sum, largest);
}
