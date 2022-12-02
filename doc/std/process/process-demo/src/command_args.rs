use std::process::Command;
use std::str;

fn main() {
    let output = Command::new("ls")
        .args(["-a", "&&", "ls", "-a"])
        .output()
        .expect("failed to execute command");
    println!("{}", str::from_utf8(output.stdout.as_slice()).unwrap())
}
