// creating a type alias to give types another name
type Kilometers = i32;

// another type alias to avoid rewriting a lengthy type and making code more readable
// Thunk stands for a Box containing a trait object that implements the Fn() and Send traits over a static lifetime
type Thunk = Box<dyn Fn() + Send + 'static>;

// this is an example of type aliases used in std::io
use std::fmt;
use std::io::Error;

type Result<T> = std::result::Result<T, std::io::Error> 

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, but: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

// using the never type
fn bar() -> ! {
    // something something
}

// another expression that has the ! type is a loop
// the loop never ends so ! is the type of the expression
// but this would not be so if we had a break statement somewhere in the loop
println!("forever ");
loop {
    println!("and ever ");
}


// str is a dynamically sized type (careful that we are not referring to &str here)
// we are unable to know how long the string is until runtime which is why we can't create
// variables of type str or take an argument of type str
let s1: str = "Hello there!";
let s2: str = "How's it going?";

// Rust needs to know how much memory to allocate for a value of a particular type
// and all values of that type need to have the same amount of memory allocated for them
// s1 would need 12 bytes, while s2 would need 15 bytes

// to fix this, we would need to make s1 and s2 &str instead of str
// because a &str only stores two values: the starting address of the string and it's length



fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // something something
    }

    fn returns_long_type() -> Thunk {
        // something something
    }
}
