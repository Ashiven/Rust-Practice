#![allow(unused_variables)]
#![allow(dead_code)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
fn main() {
    // creating a box with an i32
    let b = Box::new(5);
    println!("b = {}", b);

    // the recursive type cons list(similar to a linked list):
    // a cons list 1,2,3:  (1, (2, (3, Nil)))
    use crate::List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
