use std::collections::HashMap;
fn main() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("Content-type", "text/html");
    map.insert("Status", "ok");

    let mut s = String::new();
    for (k, v) in map.iter() {
        s.push_str(&format!("{}:{}\r\n", k, v));
    }

    print!("{}", s)
}
