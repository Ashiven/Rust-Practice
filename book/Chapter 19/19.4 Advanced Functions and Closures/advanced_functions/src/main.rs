fn add_one(x: i32) -> i32 {
    x + 1
}

// here we pass a function as parameter and return the sum of the results of two function calls
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// returning a closure
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    // add_one() gets called twice in do_twice() with the argument 5
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
    // here we could use either a closure or a function to achieve the same thing
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    // note that we are using fully qualified syntax (Trait::Method) because there are multiple
    // functions named to_string available in different traits
    let list_of_strings2: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    // enum variants are also initializer functions which is why we can also use them
    // as arguments instead of closures
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}
