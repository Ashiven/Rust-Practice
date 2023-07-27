#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt::{Debug, Display};
pub trait Summary {
    //requires implementation
    fn summarize(&self) -> String;

    //doesn't require implementation
    fn summarize_two(&self) -> String {
        format!("The content of the thing: {}", self.summarize())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
//implement the trait Summary for the NewsArticle struct
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
//implement the trait Summary for the Tweet struct
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//item will be of any type that implements the Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
//same as this
pub fn notify_two<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
//if the parameter should implement multiple traits
pub fn notify_three(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}
//we can also use the where keyword to achieve this
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

//return type should implement Summary trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you already know, people"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    //let result = largest(&number_list);
    //println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    //let result = largest(&char_list);
    //println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}

// generic structs
struct Point<T> {
    x: T,
    y: T,
}
struct Point2<A, B> {
    x: A,
    y: B,
}

// generic enums
enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// implementing methods on generic structs
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// we can also implement a method only for certain types on a generic struct
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// type parameter names go in <>
// read as: the function is generic over some type T
//fn largest<T>(list: &[T]) -> &T {
//    let mut largest = &list[0];
//
//    for item in list {
//        if item > largest {
//            largest = item;
//        }
//    }
//
//    largest
//}

// for the comparison > in the above function the type T needs to implement the trait PartialOrd

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

//we only implement the method for types T which implement the traits Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
