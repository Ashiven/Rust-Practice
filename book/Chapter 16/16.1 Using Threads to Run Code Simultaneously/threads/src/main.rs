use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // forcing termination of the spawned thread before the main logic
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // forcing termination of the spawned thread after the main logic
    // handle.join().unwrap();

    // it is important to use move closures with threads
    // because the spawned thread could live longer than the main thread
    // when we don't use a move closure, the closure will only capture a reference to v
    // (it infers that it only needs a ref to v because we is accessed in the context of println!)
    // this reference could point to deallocated memory if the main thread terminates earlier
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // this is a problem because the thread has a reference to v
    // drop(v); // when we use a move closure, we can't use v at this point because it has been moved

    handle.join().unwrap();
}
