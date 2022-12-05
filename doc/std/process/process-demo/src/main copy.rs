use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
fn main() {
    let (tx, rx) = mpsc::channel();
    let mut child = Command::new("node")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");
    // 控制台没有回显
    let mut child_stdin1 = child.stdin.take().unwrap();
    // let mut child_stdin2 = child.stdin.take().unwrap();
    let mut child_stdout1 = child.stdout.take().unwrap();
    thread::spawn(move || {
        child_stdin1.write_all(b"3+3").unwrap();
    });

    child.wait().expect("command wasn't runing");

        child.kill();
}
