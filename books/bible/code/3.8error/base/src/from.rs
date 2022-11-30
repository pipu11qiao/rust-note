#![allow(dead_code, unused_variables)]

use std::fs::File;
use std::io;

#[derive(Debug)]
struct AppError {
    kind: String,
    message: String,
}
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

fn main() -> Result<(), AppError> {
    let _file = File::open("aaa.txt")?;

    Ok(())
}
