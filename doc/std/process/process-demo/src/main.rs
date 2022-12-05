use std::io::{Write, Read};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
fn main() {
    let mut child = Command::new("node")
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");
    let child_stdout = child.stdout.take().unwrap();
    thread::spawn(move ||{
        let 
        child_stdout.read(buf)

    });

    child.wait().expect("command wasn't runing");
}
