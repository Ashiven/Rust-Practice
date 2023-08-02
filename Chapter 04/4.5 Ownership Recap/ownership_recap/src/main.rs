#![allow(unused_variables)]
#![allow(dead_code)]
fn main() {
    let words = vec!["hello".to_string()];
    let d = new_document(words);

    let words_copy = get_words(&d).to_vec();
    let mut d2 = new_document(words_copy);
    add_word(&mut d2, "world".to_string());

    assert!(!get_words(&d).contains(&"world".into()))
}

type Document = Vec<String>;

fn new_document(words: Vec<String>) -> Document {
    words
}

fn add_word(this: &mut Document, word: String) {
    this.push(word);
}

fn get_words(this: &Document) -> &[String] {
    this.as_slice()
}

/*
--recap of ownership principles--
fn main() {
  let mut a_num = 0;
  inner(&mut a_num);
}

fn inner(x: &mut i32) {
  let another_num = 1;
  let a_stack_ref = &another_num;

  let a_box = Box::new(2);
  let a_box_stack_ref = &a_box;
  let a_box_heap_ref = &*a_box;

  *x += 5;
}
*/
