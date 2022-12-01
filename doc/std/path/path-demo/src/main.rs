use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::new();

    path.push("/a");
    path.push("/b");
    path.push("/c");

    let is_dir = path.is_file();
    println!("{}",is_dir);
}
