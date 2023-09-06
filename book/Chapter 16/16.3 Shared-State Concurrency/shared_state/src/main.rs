use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Mutex::new(5);

    {
        // we have to make sure acquire the lock for the mutex before accessing it
        // the lock gets released automatically when the value goes out of scope
        let mut num = m.lock().unwrap();
        *num += 6;
    }

    println!("m = {:?}", m);

    // we have to use the Arc (atomic reference counted) type which is similar to Rc in that it
    // allows multi ownership, but it is also safe to be used across multiple threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // spawn ten threads, each of them will acquire the lock for the counter mutex and increment it by 1
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // even though counter is immutable we are getting an immutable ref
            // which means that Mutex provides interior mutability similarly to RefCell
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // force termination for each handle to make sure the value will be 10 at the end
    // if we didn't do this the main thread could terminate early and take all spawned threads with it
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
