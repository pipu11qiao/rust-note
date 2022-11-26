use std::sync::mpsc;
use std::thread;

fn main() {
    // 创建一个消息通道，返回一个元组，(发送者，接收者)
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(1).unwrap();
    });

    println!("receive {}", rx.recv().unwrap());
}
