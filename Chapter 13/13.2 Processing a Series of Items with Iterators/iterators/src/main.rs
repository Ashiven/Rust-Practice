#![allow(unused_variables)]
#![allow(dead_code)]
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // sum() takes ownership of the iterator
    // so we are not allowed to use it afterwards
    let v2 = vec![1, 2, 3];
    let v2_iter = v2.iter();
    let total: i32 = v2_iter.sum();
    assert_eq!(total, 6);

    // using map with an iterator
    // it is important to combine this with collect()
    // collect() consumes the iterator and collects resulting values in a collection
    let v2 = vec![1, 2, 3];
    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();
}
