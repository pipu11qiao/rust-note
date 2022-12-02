# Stdio

描述当传递给Command的stdin,stdout和stderror方法时，如何对子进程使用标准I/O流

## 方法

* piped() -> Stdio 应该安排一个新管道来链接父进程和子进程

使用stdout
```rust
use std::process::{Command, Stdio};

let output = Command::new("echo")
    .arg("Hello, world!")
    .stdout(Stdio::piped())
    .output()
    .expect("Failed to execute command");

assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello, world!\n");
// 控制台没有回显
```
使用stdin

```rust
use std::io::Write;
use std::process::{Command, Stdio};

let mut child = Command::new("rev")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to spawn child process");

let mut stdin = child.stdin.take().expect("Failed to open stdin");
std::thread::spawn(move || {
    stdin.write_all("Hello, world!".as_bytes()).expect("Failed to write to stdin");
});

let output = child.wait_with_output().expect("Failed to read stdout");
assert_eq!(String::from_utf8_lossy(&output.stdout), "!dlrow ,olleH");
```
如果使用.stdid(Stdio::piped())方法，将使用新的管道链接父进程和子进程。如果想要获取控制台的输出需要使用child。wait_with_output方法获取。 想要在子进程中写入需要使用
* inherit -> Stdio 子级从相应的父级描述符继承

使用inherit会从父级的标准输入和输出中读取和写入

* null -> Stdio
此流将被忽略。 这等效于将流附加到 /dev/null。

