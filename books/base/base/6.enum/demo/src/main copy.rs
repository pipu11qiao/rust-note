use std::any::type_name;

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn route(ip_kind: IpAddrKind) {}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // println!("Hello, world!");
    // let v4 = IpAddrKind::V4;
    // let v6 = IpAddrKind::V6;
    // println!("{:?}", v4);
    // print_type_of(&v6);
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
