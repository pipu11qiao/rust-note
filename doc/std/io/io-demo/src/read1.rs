use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str;

fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt")?;
    let mut buffer = [0; 3];

    let n = f.read(&mut buffer)?;


    println!("The bytes: {:?}", &buffer[..n]);

    let str = str::from_utf8(&buffer).unwrap();
    println!("{}",str);

    Ok(())
}
