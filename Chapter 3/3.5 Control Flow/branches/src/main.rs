/*
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }
}
*/

/*
--using if in a let statement--
--it is important that the return values of the expressions are of the same type--
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
}
*/

/*
--here we specifiy the return value after breaking out of the loop as counter*2--
--this is assigned to result as part of a statement--
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
*/
/*
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
*/

/*
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
*/

/*
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}")
    }
}
*/

fn main() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
