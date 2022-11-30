#![allow(dead_code, unused_variables)]

use std::fmt;

struct AppError {
    code: usize,
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_msg = match self.code {
            404 => "Sorry, Can not find the Page",
            _ => "Sorry,something is wrong! Please Try Again!",
        };
        write!(f, "{}", err_msg)
    }
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AppError {{ code: {}, message: {}}}",
            self.code, self.message,
        )
    }
}

fn produce_error() -> Result<(), AppError> {
    Err(AppError {
        code: 404,
        message: String::from("Page not found"),
    })
}

fn main() {
    match produce_error() {
        Err(e) => eprintln!("{}", e),
        _ => println!("No error"),
    }

    eprintln!("{:?}", produce_error());
    eprintln!("{:#?}", produce_error());
}
