#![allow(unused_variables)]
#![allow(dead_code)]
use std::time::Duration;
use std::{thread, vec};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // a closure with type annotations
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // the toilet closure
    let toilet_closure = |_| ();
    let s = String::from("hello");
    toilet_closure(s); // takes ownership of s and when it's stack frame is deallocated, s gets dropped

    // an ownership example
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows(); // doesn't take ownership but only takes an immutable reference
    println!("After calling closure: {:?}", list); //therefore we can still use list here

    // explicitly telling a closure to take ownership with move keyword
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    // the reason we need to move list into the closure here is because if it only took an
    // immutable ref and the main thread terminated before this, the ref would be invalid
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // sorting a list with sort_by_key(closure)
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    list.sort_by_key(|r| r.width);
    println!("{:?}", list);

    // a function that returns a closure
    // we need to use lifetimes here to tell rust that the closure returned by this function
    // does not live longer than s_ref
    // meaning we can not drop the thing s_ref refers to while the cloner or s_ref live
    //fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
    //    move || s_ref.to_string()
    //}
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
