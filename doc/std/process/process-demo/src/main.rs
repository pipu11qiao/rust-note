fn main() {
    use std::process::{Command, Stdio};

    let output = Command::new("echo")
        .arg("Hello, world!")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello, world!\n");
    // 控制台没有回显
}
