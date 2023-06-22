fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("World");
    greet(&m1, &m2);
    let s = format!("{} {}", m1, m2);
    println!("{}", s);
}

fn greet(g1: &String, g2: &String) {
    println!("{} {}", g1, g2);
}

/*
fn main() {
let mut x: Box<i32> = Box::new(1);
let a: i32 = *x;         // *x reads the heap value, so a = 1
*x += 1;                 // *x on the left-side modifies the heap value,
                         //     so x points to the value 2

let r1: &Box<i32> = &x;  // r1 points to x on the stack
let b: i32 = **r1;       // two dereferences get us to the heap value

let r2: &i32 = &*x;      // r2 points to the heap value directly
let c: i32 = *r2;    // so only one dereference is needed to read it
}
--x contains a pointer to the box on the heap, which contains the value 1--
--a will dereference x and so get the value that x points to, which is 1--
--with *x += 1 we increase the dereferenced value of by 1, making the value of the box on the heap 2--
--r1 will be a reference to x, which contains a reference/pointer to the box on the heap--
--b will dereference r1 once to get the pointer in x and twice to get the value in the box x points to--
--r2 will first dereference x to get the value in the box x points to and with the ampersand, store a reference to that--
--c will dereference r2 which is a reference/pointer to the box containing 2 and c will thus contain the value 2--
*/

/*
--implicit dereferencing--
let x: Box<i32> = Box::new(-1);
let x_abs1 = i32::abs(*x); // explicit dereference
let x_abs2 = x.abs();      // implicit dereference
assert_eq!(x_abs1, x_abs2);

let r: &Box<i32> = &x;
let r_abs1 = i32::abs(**r); // explicit dereference (twice)
let r_abs2 = r.abs();       // implicit dereference (twice)
assert_eq!(r_abs1, r_abs2);

let s = String::from("Hello");
let s_len1 = str::len(&s); // explicit reference
let s_len2 = s.len();      // implicit reference
assert_eq!(s_len1, s_len2);
*/

/*
--aliasing happens because vec.push reallocates the storage of the vector in memory, which leaves num pointing to invalid memory--
let mut vec: Vec<i32> = vec![1, 2, 3];
let num: &i32 = &vec[2];
vec.push(4);
println!("Third element is {}", *num);
--vec loses its W,O permissions when num borrows them, num gains R,O and *num gains R because of let num--
--vec.push requires R,W permissions but vec currently only has R permissions--
--when num is no longer in use it returns the original permissions to vec and loses its permissions--
*/

/*
--here s_ref will get a reference to s, which leads to s only having read permissions while s_ref lives--
--dropping s, requires s to have both read and own permissions, which it does not yet have since s_ref still lives--
let s = String::from("Hello world");
let s_ref = &s;
drop(s);
println!("{}", s_ref);
*/

/*
--the issue here is that Rust does not know for certain whether the returned reference will be default or strings--
--therefore it is possible that the return will be &default, which would then invalidate s because default gets dropped--
fn first_or(strings: &Vec<String>, default: &String) -> &String {
    if strings.len() > 0 {
        &strings[0]
    } else {
        default
    }
}

fn main() {
    let strings = vec![];
    let default = String::from("default");
    let s = first_or(&strings, &default);
    drop(default);
    println!("{}", s);
}
*/

/*
fn return_a_string() -> &String {
    let s = String::from("Hello world");
    let s_ref = &s;
    s_ref
}
--this is unsafe because s_ref will be a reference to a String which has been freed after the function returns--
--because the functions stack frame, containing s, gets deallocated after the function returns--
--the compiler will not compile that because s_ref does not have flow permissions--
*/
