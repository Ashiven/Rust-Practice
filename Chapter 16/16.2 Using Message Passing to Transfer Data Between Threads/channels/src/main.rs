use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main2() {
    // tx - transmitter  rx - receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        // the owned transmitter (move closure) sends a string on the channel in a spawned thread
        tx.send(val).unwrap()
        // we move val in send() because the receiving thread could modify it
    });

    // in the main thread we retrieve the message sent in the spawned thread via the receiver
    // recv() blocks the main thread and waits until it has received a value on the channel
    // try_recv() does not block and wait but rather returns a Result immediately(Err if no msg on channel)
    let received = rx.recv().unwrap(); // received takes ownership of the string we sent
    println!("Got: {}", received);
}

fn main() {
    let (tx, rx) = mpsc::channel();

    // to get multiple producers we just clone the transmitter and use the clones on the threads
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
