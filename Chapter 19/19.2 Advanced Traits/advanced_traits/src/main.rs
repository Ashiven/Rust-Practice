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

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
