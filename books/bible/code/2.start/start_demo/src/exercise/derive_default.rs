use std::sync::Mutex;
#[derive(Debug)]
struct Project {
    name: Mutex<String>,
    url: Mutex<String>,
}
// 填空
fn main() {
    let project = Project {
        name: Default::default(),
        url: Default::default(),
    };
    println!("{:?}", project);
}
