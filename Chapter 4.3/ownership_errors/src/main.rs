/*
fn main() {
    let name = vec![String::from("Ferris")];
    let first = &name[0];
    stringify_name_with_title(&name);
    println!("{}", first);
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}
--name is an immutable reference, but push needs write permissions, which it would have with a mutable reference--
--push would reallocate memory for the name vector and invalidate the first reference in main--
*/

fn main() {
    let name = vec![String::from("Ferris")];
    let first = &name[0];
    stringify_name_with_title(&name);
    println!("{}", first);
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}
//--here we first create a 'clone' of the input via let mut full = name.join(" ");--
//--then we modify the clone and return it, thus not having to modify the original input--

/*
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String =
      dst.iter().max_by_key(|s| s.len()).unwrap();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}
--this program is unsafe because largest is a reference to a String in dst--
--while largest lives, dst will not have write permissions, even though it is a mutable reference--
--dst has its write permissions revoked while largest is live--
--however, dst.push requires write permissions, because it could reallocate the contents of dst, thus making larges and invalid reference--
*/

/*
--a fix would consist in shortening the lifetime of largest, to not overlap with dst.push--
--this can be achieved through cloning as follows--
fn add_big_string(dst: &mut Vec<String>, src: &[String]) {
    let largest: String = dst.iter().max_by_key(|s| s.len ()).unwrap().clone();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}
*/

/*
--best would be to just copy the length of largest, since that is all we need for comparison--
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}
*/

/*
let v: Vec<i32> = vec![0, 1, 2];
let n_ref: &i32 = &v[0];
let n: i32 = *n_ref;
--this is okay--
*/

/*
let v: Vec<String> =
  vec![String::from("Hello world")];
let s_ref: &String = &v[0];
let s: String = *s_ref;
--this is not okay, because v owns the String--
--when dereferencing it, s tries to take ownership of the string and as a result both v and s would own the string--
--this would then lead to double frees--
*/
