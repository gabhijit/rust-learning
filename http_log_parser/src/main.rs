//
// A simple weblog file parser. This should help us answer simple questions
// about entries from an http log.
//

use std::path;

// BufRead is required because we are using the `lines` method
// on the BuffReader. If we don't use it, we cannot use the `lines` method
// compiler croaks.
use std::io::{self, BufRead};
use std::fs;


// We have made the return type of this function as io:Result<()>
// Because, we want to use short-cuts like `?` operator. Below
// and this requires last statemenet to be a simple Ok(()) to make the
// compiler happy.
// If we don't do that we'll have to do an ok().unwrap the result
//
// Also note: the below defilnation `io::Result<()>` is actually a bit
// interesting. Normally, we return the `std::result::Result` enum
// io::Result<T> is actually a shortcut for
// std::result::Result<T , io:Error>
//

fn main() -> io::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .expect("Please provide a filename");

    println!("Filename is {}", filename);

    if path::Path::new(&filename).exists() {
        println!("Path exists.");
    } else {
        println!("Path does not exist.");
    }

    let mut f = fs::File::open(filename)?;
    println!("Opened file is {:?}", f);

    let reader = io::BufReader::new(f);
    println!("Bufreader: {:?}", reader);

    // line below is an io::Result<String>, So we'll have to
    // extract the ok part and then unwrap it. Is there a cleaner way
    // to do it?
    for line in reader.lines() {
        println!("{}", line.ok().unwrap());
    }

    // Below will panic as unwrap is not allowed on None value
    // Only allowed on Some value
    // let none: Option<String> = None;
    // println!("{}", none.unwrap());

    Ok(())
}
