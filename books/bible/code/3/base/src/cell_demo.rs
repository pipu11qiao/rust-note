#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::Cell;

pub fn main() {
    // let c = Cell::new("asdf");
    let c = Cell::new("asdf".to_string());
    let one = c.get();
    c.set("qwer".to_string());

    let two = c.get();
    println!("{},{}", one, two);
}
