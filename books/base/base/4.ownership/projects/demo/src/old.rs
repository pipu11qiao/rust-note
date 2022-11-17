fn string_demo() {
    let mut s = String::from("hello");
    s.push_str(",world!");
    println!("{}", s)
}
fn clone_demo() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{},{},world", s1, s2);
}
fn reference_demo() {
    fn fun() {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of '{}' is {}.", s1, len);
    }
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
}

fn mut_str_demo() {
    fn main() {
        let mut s = String::from("hello");
        change(&mut s);
    }
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
}