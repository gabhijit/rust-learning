use std::sync::Mutex;

fn main() {
    // See https://doc.rust-lang.org/reference/types/closure.html#other-traits
    let locked_string: Mutex<String> = Mutex::new("hello!".to_string());

    let mut closed_string = "Hello!".to_owned();

    let mut guard = locked_string.lock().unwrap();

    std::thread::scope(|s| {
        s.spawn(|| {
            // This is allowed because guard is `Sync`
            // Because the following is a non-unique immutable borrow
            eprintln!("Guard: {:#?}", guard.len());

            // Since guard is not `Send`, this closure cannot be `Send` if following line is
            // un-commented.
            // let _ = guard.push_str("World!");

            // But since `String` is `Send`, this is allowed.
            let _ = closed_string.push_str("World!");
        });
    });

    guard.push_str("Again!");
    eprintln!("Guard: {:#?}, closed: {}", guard.len(), closed_string);
}
