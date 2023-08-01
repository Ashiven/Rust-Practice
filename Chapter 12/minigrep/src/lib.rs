use std::error::Error;
use std::fs;

// add a struct to group config values together meaningfully
// this struct now owns strings, whereas before we used references to strings
pub struct Config {
    pub query: String,
    pub file_path: String,
}

// create separate function for parsing arguments
// we have changed this into a constructor for Config to make things more idiomatic
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // return an Err variant if not enough args
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
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

    println!("With text:\n{contents}");
    Ok(()) // we return the unit type () in case of Ok
}
