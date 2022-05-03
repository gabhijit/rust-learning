use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

fn main() {
    let mut arc_map: Arc<HashMap<i32, String>> = Arc::new(HashMap::new());

    //let mut arc_map_clone = arc_map.clone();

    let join_handle = thread::spawn(move || {
        let mut inside_map = Arc::get_mut(&mut arc_map);
        if let Some(ref mut m) = inside_map {
            m.insert(2_i32, "Hello".to_string());
            eprintln!("{:#?}", inside_map);
        }
    });
    //let mut outside_map = Arc::get_mut(&mut arc_map);
    //if let Some(ref mut m) = outside_map {
    //m.insert(0_i32, "World".to_string());
    //}

    let _ = join_handle.join();
}
