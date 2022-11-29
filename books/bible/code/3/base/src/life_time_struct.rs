#![allow(dead_code)]
#![allow(unused_variables)]
struct Book<'a> {
    name: &'a str,
}
impl<'a> Book<'a> {
    fn new(name: &'a str) -> Self {
        Book { name }
    }
}

pub fn main() {
    let js_book = "js_book";

    let num = 5;

    if num > 5 {
        Book::new(js_book);
    } else {
        Book::new(js_book);
    }
    Book::new(js_book);
}
