use std::sync::Mutex;

fn main() {
    // See https://doc.rust-lang.org/reference/types/closure.html#other-traits
    let locked_string: Mutex<String> = Mutex::new("hello!".to_string());

    let mut closed_string = "Hello!".to_owned();

    let mut guard = locked_string.lock().unwrap();

    let _guard_borrow = &mut guard;

    let mut unique_immutable_string = "Hello! Unique".to_owned();

    let unique_immut_borrow = &mut unique_immutable_string;

    std::thread::scope(|s| {
        s.spawn(|| {
            // This is allowed because guard is `Sync`
            // Because the following is a non-unique immutable borrow
            eprintln!("Guard: {:#?}", guard.len());

            // borrowed guard is immutable borrow
            //let _ = (*guard_borrow).push_str("Not Allowed!");

            let _ = (*unique_immut_borrow).push_str("Allowed!");

            // Since guard is not `Send`, this closure cannot be `Send` if following line is
            // un-commented. See reference above - especially the part where a closure is `Send`.
            // > A closure is Send if all variables captured by non-unique immutable reference
            // > are Sync, and all values captured by unique immutable or mutable reference,
            // > copy, or move are Send.
            // let _ = guard.push_str("World!");

            // But since `String` is `Send`, this is allowed.
            let _ = closed_string.push_str("World!");
        });

        // Uncommenting the following line violates unique access of `unique_immut_borrow` and
        // hence is not allowed ( a compilation error!).
        // let _dont_care = &unique_immut_borrow;
    });

    guard.push_str("Again!");
    eprintln!("Guard: {:#?}, closed: {}", guard.len(), closed_string);
}
