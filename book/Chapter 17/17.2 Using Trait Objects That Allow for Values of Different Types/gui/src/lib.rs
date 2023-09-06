pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // something something
    }
}

// here we use a trait object for 'components' in the Screen struct
// Box<dyn Draw> is the trait object that acts as a stand-in for any type
// inside of a Box that implements the Draw trait
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            // we can call draw() on every component because the Screen struct
            // specifies that whatever type it is, it implements the Draw trait
            component.draw();
        }
    }
}

// if we implemented Screen like below with a generic type T, we would be limited
// to instances of Screen that have a list of components all of type 'Button' or 'TextField'
// or whatever other concrete type (it would be a homogenous collection)

// whereas with the above implementation, we could have a list of components that may hold
// a Box<Button> and a Box<TextField> and whatever other types that implement the Draw trait

/*
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
*/

// how someone else would use this gui crate

/*
use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
*/
