fn main() {
    let x = plus_one(5);

    println!("The value of x is {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

/*
--this here would be a statement and have no return value therefore which is expressed with ()--
fn plus_one(x: i32) -> i32 {
    x + 1;
}
*/
