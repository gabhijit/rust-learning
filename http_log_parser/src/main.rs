//
// A simple weblog file parser. This should help us answer simple questions
// about entries from an http log.
//

use std::path;

// BufRead is required because we are using the `lines` method
// on the BuffReader. If we don't use it, we cannot use the `lines` method
// compiler croaks. Also this {self} for std::io:: means we can use
// std::io (the top level module)
use std::io::{self, BufRead};
use std::fs;

use std::vec::IntoIter;
use std::collections::HashMap;

// Deriving debug trait for {:?} printing
#[derive(Debug)]
struct HTTPLogEntry {
    ipaddress: String,
    url: String,
    status_code: u16,
    transferred: u32
}

// In the following function we could return a Vector of &HTTPLogEntry,
// But the problem with that is, `entry` that we push into the vector is
// scoped only to the function (local variable), so passing the reference
// to that and returning that won't be possible. We can indeed do that if
// we allocate entries on Heap and then passed their reference. That can
// be done by implementing `new` for the HTTPLogEntry. We've not done that,
// but we can try doing that.
//
// Optionally we can pass 'consuming' Vec<HTTPLogEntry> where when we 'push'
// the entry, a copy is made and it is passed (so something like `new` above
// is done for us by the compiler.
//
// In general though, we'd typically, iterate over such entries, so it's perhaps
// a good idea to make this into an iterator and pass that, which is what
// we are using below.
//
// Also a note about this whole AsRef<path::Path>, Basically this means, this
// function will work on anything that will take a Path as an argument. A normal
// String, &str (A string slice) can be converted into a Path and hence just work
// as easy.
//
// We are returning io::Result for the same reasons, we want to use the short-cut
// '?' operator on most I/O.
fn parse_weblog_file<P: AsRef<path::Path>>(filename: P) -> io::Result<IntoIter<HTTPLogEntry>> {

    let mut http_entries = Vec::new();

    let f = fs::File::open(filename)?;
    println!("Opened file is {:?}", f);

    let reader = io::BufReader::new(f);
    println!("Bufreader: {:?}", reader);

    // line below is an io::Result<String>, So we'll have to
    // extract the ok part and then unwrap it. Is there a cleaner way
    // to do it?
    for line in reader.lines() {
        let line = line.ok().unwrap();
        let tokens: Vec<&str> = line.split(' ').collect();
        let ipaddress = String::from(tokens[0]);
        let url = String::from(tokens[6]);
        let status_code = tokens[8];
        let transferred = tokens[9];

        let entry = HTTPLogEntry {
            ipaddress,
            url,
            status_code: status_code.parse().unwrap(),
            transferred: transferred.parse().unwrap_or(0)
        };

        http_entries.push(entry);

    }

    Ok(http_entries.into_iter())
}


// We have made the return type of this function as io:Result<()>
// Because, we want to use short-cuts like `?` operator. Below
// and this requires last statemenet to be a simple Ok(()) to make the
// compiler happy.
// If we don't do that we'll have to do an ok().unwrap the result
//
// Also note: the below defilnation `io::Result<()>` is actually a bit
// interesting. Normally, we return the `std::result::Result` enum
// io::Result<T> is actually a shortcut for
// std::result::Result<T , io::Error>
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

    // A note about the following. I haven't got it yet. But
    // If I try to make a hashmap of &str or &String, it's a bit involved
    // It's better to make a HashMap that just 'owns' the corresponding
    // values. May be after I figure out lifetimes, I can fix this.
    // Idiomatic? Don't know.
    let mut urls_entries: HashMap<String, u32> = HashMap::new();
    let mut ips_entries: HashMap<String, u32> = HashMap::new();
    let mut ips_data_xferred: HashMap<String, u32> = HashMap::new();

    let entries = parse_weblog_file(filename)?;
    for entry in entries {

        let urls_count = urls_entries.entry(entry.url.clone()).or_insert(0);
        *urls_count += 1;

        let ip_count = ips_entries.entry(entry.ipaddress.clone()).or_insert(0);
        *ip_count += 1;

        let data_xferred = ips_data_xferred.entry(entry.ipaddress.clone()).or_insert(0);
        *data_xferred += entry.transferred;

        //println!("{:?} ", entry);
    }

    let mut entries: Vec<(&String, &u32)> = ips_entries.iter().collect();
    entries.sort_by(|a,b| a.1.cmp(b.1).reverse());
    entries.truncate(5);
    println!("{:?}", entries);

    let mut entries: Vec<(&String, &u32)> = urls_entries.iter().collect();
    entries.sort_by(|a,b| a.1.cmp(b.1).reverse());
    entries.truncate(5);
    println!("{:?}", entries);

    let mut entries: Vec<(&String, &u32)> = ips_data_xferred.iter().collect();
    entries.sort_by(|a,b| a.1.cmp(b.1).reverse());
    entries.truncate(5);
    println!("{:?}", entries);

    // Below will panic as unwrap is not allowed on None value
    // Only allowed on Some value
    // let none: Option<String> = None;
    // println!("{}", none.unwrap());

    Ok(())
}
