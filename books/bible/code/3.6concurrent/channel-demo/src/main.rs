#![allow(dead_code)]
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        tx.send("hi from raw tx".to_string()).unwrap();
    });
    thread::spawn(move || {
        tx1.send("hi from clone tx".to_string()).unwrap();
    });

    for recieved in rx {
        println!("Got: {}", recieved);
    }
}
fn main_for() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for recieved in rx {
        println!("Got: {}", recieved);
    }
}
fn main_try_recv() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(1).unwrap();
    });
    println!("receive {:?}", rx.try_recv());
    println!("receive {:?}", rx.try_recv());
    println!("receive {:?}", rx.try_recv());
}

fn main_owner() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let s = String::from("我，飞走喽！");
        tx.send(s).unwrap();
        // println!("val is {}", s);
    });
    let recieved = rx.recv().unwrap();
    println!("Got: {}", recieved);
}

fn main_tx_rx() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        tx.send(1).unwrap();
        println!("thread run");
        println!("thread run 2");
        thread::sleep(Duration::from_secs(2));
        println!("thread run 3");
    });
    println!("receive {}", rx.recv().unwrap());
    thread::sleep(Duration::from_secs(3));
}
