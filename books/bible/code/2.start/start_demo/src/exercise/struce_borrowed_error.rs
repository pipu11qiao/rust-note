use std::collections::HashMap;

#[derive(Debug, Clone)]
struct A<'a> {
    name: &'a str,
    age: Option<String>,
}
impl<'a> A<'a> {
    fn name(&self) -> &str {
        &self.name
    }
}
fn main() {
    let a = A {
        name: "name",
        age: Some("age".to_string()) ,
    };

    let b = a.clone();
    println!("{:?}", b);

    let string = format!("{},{}", b.name(), b.age.unwrap().as_str());
    println!("{}", string);
}
