use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    //adding key-value pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //let team_name = String::from("Blue");

    //retrieving values
    //let score = scores.get(&team_name).copied().unwrap_or(0);

    //iterating over key-value pairs
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    //updating values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    //inserting a key-value only if the key does not have a value associated with it
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);

    println!("{:?}", scores);

    //counting frequencies
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
