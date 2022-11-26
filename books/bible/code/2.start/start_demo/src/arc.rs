use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    let count = Arc::new(Mutex::new(3));

    let counter = Arc::clone(&count);

    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num = 5;
    });
    handle.join().unwrap();

    print!("{:?}", *count.lock().unwrap())
}
