use std::fs;
use std::error::Error;
use std::env;

// We need to make both `query` and `filename` public
pub struct Config {
    pub query: String,
    pub filename: String,
    case_insensitive: bool

}

impl Config {

    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("Not Enough Arguments!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

        return Ok(Config { query, filename, case_insensitive });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    println!("Searching {} in {}", config.query, config.filename);

    // Use of `?` operator returns error if the result is an error.
    let contents = fs::read_to_string(config.filename)?;

    // We assign `search_fn` to a value depending upon the value of
    // config.case_insensitive. Keep in mind that "if else blocks are
    // expressions, so they can 'return a value, which can also be
    // a function. :-)
    let search_fn = if config.case_insensitive {
        search
    } else {
        search_case_insensitive
    };

    for line in search_fn(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}


fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {

    let query = query.to_lowercase();

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
Safe, Fast, Productive.
Pick three.";

        assert_eq!(vec!["Safe, Fast, Productive."], search(query, contents));

    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
Safe, Fast, Productive.
Pick Three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."],
                   search_case_insensitive(query, contents));
    }

}
