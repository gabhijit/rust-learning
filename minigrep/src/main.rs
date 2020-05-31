use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem in Parsing arguments: {}", err);
        process::exit(1);
    });

    // if let is more 'idiomatic' because we are doing nothing
    // with the `Ok` value. So it's a good idea to check for
    // existence of error using if let
    if let Err(err) = minigrep::run(config) {
        eprintln!("Error running: {}", err);
        process::exit(2);
    }

    // In place of above, we could've used the more verbose
    // form
    // if _ = run(config).unwrap_or_else(|err| { ...});

}
