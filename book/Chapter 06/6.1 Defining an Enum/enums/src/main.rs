#![allow(unused_variables)]
#![allow(dead_code)]

// V4 and V6 are of type IpAddrKind
enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        //something
    }
}
// could be expressed with the following structs as well
// problem here is that they are not grouped under one type
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));
    let m = Message::Write(String::from("hello"));
    m.call();
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
}
fn route(ip_kind: IpAddrKind) {}

fn main() {
    println!("hello world");
}
