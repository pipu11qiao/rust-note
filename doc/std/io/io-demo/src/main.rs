use std::io::{self, Read};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let mut child = Command::new("npm")
        .current_dir("/Users/wangyong/Tmp/webpack5-base")
        .args(["run", "build"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute node");

    println!("333333333");
    let t = thread::spawn(move || {
        let count = 60;
        let mut times = 0;
        loop {
            times += 1;
            if (times > count) {
                break;
            }
            let mut buffer = String::new();

            let mut child_out = child.stdout.as_mut().unwrap();
            child_out.read_to_string(&mut buffer).unwrap();
            println!("buffer is {} ", buffer);
            thread::sleep(Duration::from_millis(50))
        }

        child.kill();
    });
    t.join();
    // child.wait();
}
