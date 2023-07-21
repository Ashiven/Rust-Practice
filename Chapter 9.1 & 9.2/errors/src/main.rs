use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
fn main() {
    println!("Hello, world!");

    //explicitly calling panic
    //panic!("crash and burn");

    //panic called by a library function
    //let v = vec![1, 2, 3];
    //v[99];

    // using unwrap for primitive error handling because unwrap also calls panic! on error
    let greeting_file = File::open("hello.txt").unwrap();

    // using expect for error handling with custom messages, otherwise same as unwrap
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // error handling upon opening a file
    // when we get a file not found error we create the file
    // on a different error, we just panic and output the error
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

// propagating errors to calling functions
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// a shorter way of propagating errors with the ? operator
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// even shorter
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// shortest of them all
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
