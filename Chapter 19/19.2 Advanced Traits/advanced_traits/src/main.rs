// defining a trait with an associated type 'Item'
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// implementing this trait
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option(Self::Item) {
        // implementation
    }
}

// the problem with using generics for this type definition
// would be that you would have to specify the specific type on every trait implementation
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

// you would have to specify the type for every implementation of Iterator<T>
// and therefore you could have multiple implementations of the same trait for different types
impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        // implementation
    }
}

// operator overloading (implementing traits of standard operations)
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// here we implement the Add trait for the Point struct, which makes it so that we can add two Points via +
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// this is the definition of the Add trait
// as we can see the default placeholder for the generic type Rhs (right hand side) is Self
// then, we also have an associated type Output, which the implementor specifies for the type of the output
trait Add<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

struct Millimeters(u32);
struct Meters(u32);

// here we will actually specify a type that differs from the default generic type placeholder
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Self::Output {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// implementing multiple traits with the same method name
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// if we want to define a trait that can only be implemented on types that implement another trait
// (sort of a prerequisite to be able to implement this trait), we can use this syntax (specify a supertrait)

use std::fmt;

// this means that OutlinePrint can only be implemented for types that also implement the fmt::Display trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string(); // to_string() is implemented for any type implementing the fmt::Display trait
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// if we tried to implement this trait for 'Point' before also implementing the fmt::Display trait for Point, we would get an error
impl OutlinePrint for Point {}

// lets say we want to implement a trait on a type and neither the trait nor the type are locally defined in our crate
// we can still do so if we use a so called 'Wrapper'

struct Wrapper(Vec<String>);

// here we can now implement the Display trait on a Vector of Strings even though neither the type Vector or the trait Display are local
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // since we have implemented a fly() method on human, this is what the compiler defaults to
    let person = Human;
    // there are three implementations of fly and we default to the one on the struct
    person.fly();
    // if we want to call the ones we defined in the traits, we need to specify which one we call
    Pilot::fly(&person);
    Wizard::fly(&person);

    // if we want to use the baby_name implementation for Dog that we defined in the 'Animal' trait
    // we have to use the following syntax because baby_name() does not take a &self parameter
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // since we have implemented the Display trait for the Wrapper(Vec<String>), we can now use it in a println! macro
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
