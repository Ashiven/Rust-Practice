#![allow(unused_variables)]
#![allow(dead_code)]

// this is a normal struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// this is a tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// this is a unit struct
// this becomes more relevant for traits later on
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    user1.email = String::from("anotheremail@example.com");

    // we can update a Struct like this, taking over all values from user1 besides email
    // important to note that this moves ownership to user2 and we can't use user1 any longer
    // this is because the struct contains String types which transfer ownership
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

// instead of username: username and email: email we can write username, email, ...
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    println!("Hello World");
}
