use std::thread;
use std::time::{Duration, SystemTime};

fn main() {
    // let now = SystemTime::now();
    let handle = thread::spawn(move || {
        let now = SystemTime::now();
        let mut index = 1;
        // while index < 10 {
        //     thread::sleep(Duration::from_millis(20));
        //     println!("index is {index}");
        //     index += 1;
        // }
        thread::sleep(Duration::from_millis(200));
        let new_now = SystemTime::now().duration_since(now);
        let num = new_now.unwrap().as_millis();
        println!("time is {}", num);
    });

    handle.join().unwrap();

    // for _i in 1..5 {
    //     index += 1;
    //     println!("hi number {} from main thread", index);
    //     thread::sleep(Duration::from_millis(1))
    // }
}
