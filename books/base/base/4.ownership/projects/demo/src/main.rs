fn string_demo() {
    let mut s = String::from("hello");
    s.push_str(",world!");
    println!("{}", s)
}
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{},world", s1);
}
