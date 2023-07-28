#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt::Display;
// we can also use lifetimes on structs
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // string3 is the parameter with the longer lifetime
    // therefore a' will have the concrete lifetime of string4
    // which will then also be the lifetime of result
    // if we moved the println! out of that scope we would get a compiler error
    // because result does not live long enough
    let string3 = String::from("long string is long");
    {
        let string4 = String::from("xyz");
        let result = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result);
    }

    // using a struct with lifetimes
    // first_sentence is a string slice, which is a reference to the String novel
    // what we are saying is that the struct can't outlive the reference it holds in the part field
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // string literals have the 'static lifetime
    // because they are stored directly in the programs binary and therefore they are always available
    let s: &'static str = "I have a static lifetime.";
}

// we need to specfiy a lifetime here
// meaning: the returned reference will be valid as long as both the parameters are valid
// the generic lifetime 'a will get the smaller of the concrete lifetimes of x and y as concrete lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// we are definining a function for slices x and y with a lifetime of 'a, such that
// the return slice will receive the shorter of the lifetimes of x and y
// furthermore, we implement this function for generic types T, with an input parameter
// ann, which is of type T and we say that T must implement the Display trait
// because T implements the Display trait we can use ann in the println! statement with {}
// generics go into the angle brackets right after the function name
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
