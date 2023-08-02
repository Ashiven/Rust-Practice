#![allow(unused_variables)]
#![allow(dead_code)]
fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

/*
--this would not work because ownership has been moved from 'first' to 'full' and can not be used any longer therefore--
fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
 */

/*
--can be fixed by using clone()--
--with first.clone() we allocate another box on the heap containing 'Ferris' which will be pointed to and owned by 'first_clone'--
fn main() {
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
*/
