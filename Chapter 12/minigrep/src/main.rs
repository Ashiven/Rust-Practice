use std::{env, process};

use minigrep::Config;
fn main() {
    // getting the user supplied arguments
    // using collect to turn the iterator into a vector
    let args: Vec<String> = env::args().collect();

    // saving the relevant args inside of variables
    // some error handling if the constructor returns an Err variant
    // we print the contents of the error and explicitly exit the program with return code 1
    // unwrap_or_else only activates if unwrap encounters an Err variant and then executes the given code, passing the contents of Err to it
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.file_path);

    // error handling with if let
    // only if run(config) returns the matching on the left, the code inside of the brackets gets executed
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
