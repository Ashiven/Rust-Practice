#![allow(unused_variables)]
#![allow(dead_code)]
fn main() {
    let data = "initial contents";

    //converting string literals to strings
    let s = data.to_string();
    let s = "initial contents".to_string();

    //we can also use String::from to convert a string literal to a string
    let s = String::from("initial contents");

    //appending a string slice to a string with push_str
    let mut s = String::from("foo");
    s.push_str("bar");

    //we give a string slice to push_str because we don't want to take ownership of the parameter
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    //adding a single character to a string with the push method
    let mut s = String::from("lo");
    s.push('l');

    //concatenating strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; //s1 gets moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    //slicing strings
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    //iterating over the characters of a string rather than it's bytes
    for c in "Здравствуйте".chars() {
        println!("{c}");
    }

    //iterating over a strings bytes
    for b in "Здравствуйте".bytes() {
        println!("{b}");
    }
}
fn main() {
    println!("hello world");
}
