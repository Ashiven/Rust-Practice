use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// implementing the Deref trait for MyBox
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 // returning a reference to the first value in the tuple struct
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // y can't be dereferenced because we haven't implemented the Deref trait for MyBox

    // we pass a reference to our MyBox type, which will be made into &String by our deref implementation when it gets dereferenced
    // hello expects a &str , which Rust handles via its implementation of the Deref trait for String that returns a &str
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
