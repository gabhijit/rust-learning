use regex::Regex;

fn main() {
    // Testing for a failing regular expression - using `-u` helps in making sure this re gets
    // compiled.
    let tag = Regex::new(r"(?-u:[\w][\w.-]{0,127})").unwrap();

    let _ = tag.is_match("foobar");
}
