use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    bad_arc_example();
    bad_arc_example_improved();
    good_arc_example_pass1();
    good_arc_example();
}
// In the following bad_arc_example, both the maps `inside_map` (spawned in thread ) as well as
// `outside_map` (from main thread) will return `None` in `get_mut`. This is because the map is
// shared by two threads.
fn bad_arc_example() {
    eprintln!("bad_arc_example:");
    let mut bad_arc_map: Arc<HashMap<i32, String>> = Arc::new(HashMap::new());

    let mut bad_arc_map_clone = Arc::clone(&bad_arc_map);
    let join_handle = thread::spawn(move || {
        let mut inside_map = Arc::get_mut(&mut bad_arc_map);
        eprintln!("inside_map: {:#?}", inside_map);
        if let Some(ref mut m) = inside_map {
            m.insert(2_i32, "Hello".to_string());
            eprintln!("{:#?}", inside_map);
        }
    });
    let mut outside_map = Arc::get_mut(&mut bad_arc_map_clone);
    if let Some(ref mut m) = outside_map {
        m.insert(0_i32, "World".to_string());
    }
    eprintln!("outside_map: {:#?}", outside_map);

    let _ = join_handle.join();
}

// In the following `bad_arc_example_improved`, the main thread is able to successfully get `Some`
// value in the `get_mut`. This happens because we `join` for the spawned thread first, thus making
// sure that the spawned thread is completed.
fn bad_arc_example_improved() {
    eprintln!("bad_arc_example_improved:");
    let mut bad_arc_map: Arc<HashMap<i32, String>> = Arc::new(HashMap::new());

    let mut bad_arc_map_clone = Arc::clone(&bad_arc_map);
    let join_handle = thread::spawn(move || {
        let mut inside_map = Arc::get_mut(&mut bad_arc_map);
        eprintln!("inside_map: {:#?}", inside_map);
        if let Some(ref mut m) = inside_map {
            m.insert(2_i32, "Hello".to_string());
            eprintln!("{:#?}", inside_map);
        }
    });
    // following line makes sure that the 'spawned' thread is completed.
    let _ = join_handle.join();

    let mut outside_map = Arc::get_mut(&mut bad_arc_map_clone);
    if let Some(ref mut m) = outside_map {
        m.insert(0_i32, "World".to_string());
    }
    eprintln!("outside_map: {:#?}", outside_map);
}

// `good_arc_example_pass1` shows how to 'update' the map from both the `spawned` thread and the `main`
// thread. Note: Use of `Mutex` (`Mutex` and similarly `RwLock` are atomic that is they `succeed`
// or `block`).
//
fn good_arc_example_pass1() {
    eprintln!("good_arg_example_pass1:");
    let good_arc_map: Arc<Mutex<HashMap<i32, String>>> = Arc::new(Mutex::new(HashMap::new()));

    let good_arc_map_clone = Arc::clone(&good_arc_map);
    let join_handle = thread::spawn(move || {
        eprintln!("I:1");
        let mut inside_map = good_arc_map.lock();
        eprintln!("I:2");
        if let Ok(ref mut m) = inside_map {
            m.insert(2_i32, "Hello".to_string());
        }
    });

    // This code can potentially deadlock if the main thread gets to run first (and it will almost
    // always because it is already running). Why the 'deadlock' is caused? It is because -
    // `outside_map` is a `MutexGuard` that is not dropped, till the end of the main thread.
    //
    // We can avoid this deadlock if we 'delay' execution of the main thread and thus allowing the
    // spawned thread to run to completion and then update the map in the main thread.

    // commenting following line causes deadlock. You will almost always observe this.
    thread::sleep(Duration::from_millis(10));

    eprintln!("O:1");
    let mut outside_map = good_arc_map_clone.lock();
    eprintln!("O:2");
    if let Ok(ref mut m) = outside_map {
        m.insert(0_i32, "World".to_string());
    }

    let _ = join_handle.join();
    eprintln!("final_map: {:#?}", outside_map);
}

// `good_arc_example` shows how to 'update' the map from both the `spawned` thread and the `main`
// thread. Here we are trying to make sure the `MutexGuard` returned is dropped thus allowing the
// other to get `Some` value in `get_mut`.
fn good_arc_example() {
    eprintln!("good_arg_example:");
    let good_arc_map: Arc<Mutex<HashMap<i32, String>>> = Arc::new(Mutex::new(HashMap::new()));

    let good_arc_map_clone = Arc::clone(&good_arc_map);
    let join_handle = thread::spawn(move || {
        eprintln!("I:1");
        let mut inside_map = good_arc_map.lock();
        eprintln!("I:2");
        if let Ok(ref mut m) = inside_map {
            m.insert(2_i32, "Hello".to_string());
        }
    });

    // Creating a block to make sure that outside_map is 'really short lived' and thus avoids the
    // 'deadlock'. We need to make sure that `outside_map` is 'dropped, otherwise it won't allow
    // `inside_map` to update ever.
    {
        eprintln!("O:1");
        let mut outside_map = good_arc_map_clone.lock();
        eprintln!("O:2");
        if let Ok(ref mut m) = outside_map {
            m.insert(0_i32, "World".to_string());
        }
    }

    let _ = join_handle.join();

    // But since `outside_map` is dropped, already we need to get the reference to the map again
    // through `Mutex` (by calling `.lock().unwrap()`
    eprintln!("final_map: {:#?}", good_arc_map_clone.lock().unwrap());
}
