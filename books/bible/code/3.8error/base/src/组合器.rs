#![allow(dead_code, unused_variables)]

fn main_unwrap_or_default() {
    let s1 = Some(5);
    s1.unwrap_or_default();
    let o1: Result<i32, i32> = Ok(5);
}
fn main_unwrap_or() {
    let s1 = Some(5);
    s1.unwrap_or(2);
    let o1: Result<i32, i32> = Ok(5);
    o1.unwrap_or(3);
}
fn main_unwrap() {
    let s1 = Some(5);
    s1.unwrap();
    let o1: Result<i32, i32> = Ok(5);
    o1.unwrap();
}
fn main_ok_or_ok_or_else() {
    const ERR_DEFAULT: &str = "error message";

    let s = Some("abcde");
    let n: Option<&str> = None;

    let o: Result<&str, &str> = Ok("abcde");
    let e: Result<&str, &str> = Err(ERR_DEFAULT);

    assert_eq!(s.ok_or(ERR_DEFAULT), o);
    assert_eq!(n.ok_or(ERR_DEFAULT), e);

    assert_eq!(s.ok_or_else(|| "error message"), o);
    assert_eq!(n.ok_or_else(|| "error message"), e);
}
fn main_map_or_else() {
    let s: Result<u32, ()> = Ok(10);
    let n: Option<u32> = None;
    let fn_closure = |v: u32| v + 2;

    assert_eq!(s.map_or_else(|_| 3, fn_closure), 12);
    assert_eq!(n.map_or_else(|| 3, fn_closure), 3);
}

fn main_map_or() {
    const V_DEFAULT: u32 = 1;

    let s: Result<u32, ()> = Ok(10);
    let n: Option<u32> = None;
    let fn_closure = |v: u32| v + 2;

    assert_eq!(s.map_or(V_DEFAULT, fn_closure), 12);
}

fn main_map_map_erro() {
    let s1 = Some("abcde");
    let s2 = Some(5);

    let n1: Option<&str> = None;
    let n2: Option<usize> = None;

    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<usize, &str> = Ok(5);

    let e1: Result<&str, &str> = Err("abcde");
    let e2: Result<&str, i32> = Err(404);

    let fn_character_count = |s: &str| s.chars().count();

    assert_eq!(s1.map(|_| 5), s2);
    assert_eq!(e1.map_err(|_| 404), e2);
}

fn main_filter() {
    let s1 = Some(3);
    let s2 = Some(6);
    let n = None;

    let fn_is_even = |x: &i8| x % 2 == 0;

    assert_eq!(s1.filter(fn_is_even), n);
    assert_eq!(s2.filter(fn_is_even), s2);
    assert_eq!(n.filter(fn_is_even), n);
}

fn main_or_else() {
    let s1 = Some("some1");
    let s2 = Some("some2");
    let fn_some = || Some("some2");

    let n: Option<&str> = None;
    let fn_none = || None;

    assert_eq!(s1.or_else(fn_some), s1);
    assert_eq!(s1.or_else(fn_none), s1);
    assert_eq!(n.or_else(fn_none), s2);
    assert_eq!(n.or_else(fn_none), n);

    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let fn_ok = |_| Ok("ok2");

    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");
    let fn_err = |_| Err("error2");

    assert_eq!(o1.or_else(fn_ok), o1); // Ok1 or_else Ok2 = Ok1
    assert_eq!(o1.or_else(fn_err), o1); // Ok1 or_else Ok2 = Ok1
}
fn main_or_and() {
    let s1 = Some("some1");
    let s2 = Some("some2");
    let n: Option<&str> = None;

    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");

    assert_eq!(s1.or(s2), s1);
    assert_eq!(s1.or(n), s1);
    assert_eq!(n.or(s1), s1);
    assert_eq!(n.or(n), n);

    assert_eq!(o1.or(o2), o1);
    assert_eq!(o1.or(e1), o1);
    assert_eq!(e1.or(o1), o1);
    assert_eq!(e1.or(e2), e2);

    assert_eq!(s1.and(s2), s2);
    assert_eq!(s1.and(n), n);
    assert_eq!(n.and(s1), n);
    assert_eq!(n.and(n), n);

    assert_eq!(o1.and(o2), o2);
    assert_eq!(o1.and(e1), e1);
    assert_eq!(e1.and(o1), e1);
    assert_eq!(e1.and(e2), e1);
}

use std::fmt;
#[derive(Debug)]
struct AppError;

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An Error Occurred,Please Tray Again!")
    }
}

fn produce_error() -> Result<(), AppError> {
    Err(AppError)
}

fn main() {
    match produce_error() {
        Err(e) => println!("{}", e),
        _ => println!("No error"),
    }
    eprintln!("{:?}", produce_error());
}
