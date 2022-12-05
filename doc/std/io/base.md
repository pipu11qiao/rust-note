# std::io

std::io模块包含了许多在执行输入和输出时需要的常见操作。该模块中最核心的部分是Read和Write traits, 它们提供了用于读取和写入输入和输出的最通用接口。

## 读和写

因为它们是 traits，所以 Read 和 Write 由许多其他类型实现，您也可以为您的类型实现它们。 这样，您将在此模块的整个文档中看到几种不同类型的 I/O: File，TcpStream，有时甚至是 Vec<T>。 例如，Read 添加了 read 方法，我们可以在 File 上使用该方法：


Read和Write非常重要，两个traits的实现者有一个昵称：readers和writers. 因此有时会看到reader而不是实现Read trait的类型

## Seek和BufRead

除此之外,还提供了重要的trasits: Seeh和BufRead.
```rust
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::str;

fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt")?;
    let mut buffer = [0; 10];

    //
    f.seek(SeekFrom::End(-10))?;

    let n = f.read(&mut buffer)?;

    println!("The bytes: {:?}", &buffer[..n]);

    let str = str::from_utf8(&buffer).unwrap();
    println!("{}", str);
    let str1 = "人";

    let buf1 = str1.as_bytes();
    println!("{:?}", buf1);

    println!("{}", str::from_utf8(&buf1).unwrap());
    Ok(())
}
```

## BufReader 和 BufWriter

基于字节的接口笨拙且效率低下，因为我们需要对操作系统进行近乎恒定的调用。为了解决这个问题，std::io带有两个结构体BufReader和BufWriter,他们包装了readers和writers。包装器使用缓冲区，从而减少了调用次数，并提供了更好的方法来访问所需的内容。

```rust
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let f = File::open("foo.txt")?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    // read line
    reader.read_line(&mut buffer)?;
    // reader.read_line(&mut buffer)?;

    println!("{}", buffer);
    Ok(())
}

```

## 标准输入 输出

输入的一个非常常见的来源是标准输入：

```rust
use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    println!("You typed: {}", input.trim());
    Ok(())

}
```