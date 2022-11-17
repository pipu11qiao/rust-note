#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    // println!("Hello, world!");
    // let v4 = IpAddrKind::V4;
    // let v6 = IpAddrKind::V6;
    // println!("{:?}", v4);
    // print_type_of(&v6);
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}
enum Option<T> {
    None,
    Some(T),
}
