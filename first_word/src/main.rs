fn main() {
    println!("Hello, world!");

    let mut s = String::from("hello, world!");

    let word = first_word(&s);

    println!("First word is {}!", word);

    s.clear();

    let s = "Welcome Again!";

    println!("First word of {} is {}.", s, first_word(s));
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
