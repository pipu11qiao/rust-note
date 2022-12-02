# std::process

用于处理进程的模块
该模块主要鱼产生和与子进程交互有关，但是它也提供了abort和exit来终止当前进程。


### 产生一个进程

```rust
use std::process::Command;

fn main() {
    let output = Command::new("echo")
        .arg("Hello world")
        .output()
        .expect("Failed ot execute command");
    assert_eq!(b"Hello world\n", output.stdout.as_slice());
}

```

Command上的集中方法（例如spawn和output)可用于spawn进程，特别是ouptput生成子进程病等待直到该进程终止,而spaswn将返回代表生成的子进程的child


### 处理I/O

可以通过将Stdio传递给Command上的相应方法来配置子进程的stdout、stdin和stderr,生成后可以从Child访问他们。
例如，可以将一个命令的输出管道输送到另一个命令，如下所示：

```rust
use std::process::{Command, Stdio};
use std::str;

fn main() {
    let echo_child = Command::new("echo")
        .arg("Oh no, a tpyo!")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process");

    let echo_out = echo_child.stdout.expect("Failed to oepn echo stdout");

    let sed_child = Command::new("sed")
        .arg("s/tpyo/typo/")
        .stdin(Stdio::from(echo_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start sed process");

    let output = sed_child.wait_with_output().expect("Failed to wait on sed");
    let str = str::from_utf8(&output.stdout).unwrap();
    println!("{}", str);
    assert_eq!(b"Oh no, a typo!\n", output.stdout.as_slice());
}

```

请注意，ChildStdErr和ChildStdout实现Read,而ChildStdin实现Write

```rust
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let mut child = Command::new("/bin/cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");
    // 如果子进程填充了其 stdout 缓冲区，则它可能最终会等待，直到父进程读取 stdout，并且在此期间无法读取 stdin，从而导致死锁。
    //
    // 从另一个线程进行写入可确保同时读取 stdout，从而避免了该问题。
    //
    //

    let mut stdin = child.stdin.take().expect("failed to get stdin");

    std::thread::spawn(move || stdin.write_all(b"test").expect("failed to write to stdin"));

    let output = child.wait_with_output().expect("failed to wait no child");

    assert_eq!(b"test", output.stdout.as_slice());
}

```
