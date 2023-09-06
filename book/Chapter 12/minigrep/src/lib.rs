use std::env;
use std::error::Error;
use std::fs;

// add a struct to group config values together meaningfully
// this struct now owns strings, whereas before we used references to strings
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

// create separate function for parsing arguments
// we have changed this into a constructor for Config to make things more idiomatic
impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // if the environment variable IGNORE_CASE is set, we set the ignore_case field of the Config struct to true
        // the result of the env::var function is a Result enum which is of variant Ok if the env variable has been set to any value
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// which also returns a Result type for better error handling in main
// we moved the logic into a separate function
// Box<dyn Error> is any type that implements the Error trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // reading the contents of the specified file
    // using expect instead of unwrap for custom error messages
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    //println!("With text:\n{contents}");
    Ok(()) // we return the unit type () in case of Ok
}

// the data returned by this function will live as long as the data passed in the contents parameter
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct Tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
