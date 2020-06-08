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

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers: Vec<i32> = vec![134, 50, 25, 100, 65];

    // If I uncomment the following, I am not allowed to do so because, once I borrow, I am not
    // allowed to `move out` below in the `into_iter`. I guess, I should be able to fix this with
    // the liftimes?
    //let largest = largest(&numbers);

    // `into_iter()` is important one, if we simply use `iter()`, it iterates over references that
    // is `&i32`. Now the problem with that is. This will require to change the implementation
    // above to use `iter.fold(0.0, |sum, &x| sum + x as f32 );`, but this will not agree with the
    // definition of `fold` method of the iterator, because `&x` cribs, we need iterator over
    // `elements` and not `reference to elements` and that is provided by `into_iter` so we simply
    // use that.
    let sum: f32 = MySum::sum(numbers.into_iter());

    println!("The sum is {:?}", sum);
    //println!("The sum is {:?}, largest number is {:?}", sum, largest);
}
