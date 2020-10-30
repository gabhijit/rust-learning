#![allow(unused_variables)]
fn main() {

    let mut s = String::from("hello");

    // The reference is an immutable reference
    // A reference to an immutable string does not simply make
    // the reference immutable, you've to explicitly make that
    // reference immutable.
    change(&mut s);

    let word = first_word(&s);

    println!("First word is {}!", word);

    // This is allowed 'after' word because word is no longer used
    // So mutable reference is just fine.
    s.clear();

    // Following 's' is a totally new variable and just works
    // This has got nothing to do with former 's'. (shadowed)
    let s = "Welcome Again!";

    println!("First word of {} is {}.", s, first_word(s));

    // This is another 's'
    let s = String::from("Hello World!");

    // A mutable reference to immutable string literal,
    // of course won't work. Try uncommenting line below and compiling
    // change(&mut s);
}

// Changing the input parameter to `&str` would make our API more
// general. This will allow us to do something like say -
// first_word("Welcome Again!")
// Because String literals are string slices denoted by a type
// `&str`
fn first_word(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {

        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]

}

fn change(s: &mut String) {

    s.push_str(", World!");

}
