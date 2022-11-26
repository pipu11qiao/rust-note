fn main() {
    let path = "open/ok/tx.html";
    let route: Vec<&str> = path.split("/").collect();
    let s = match route[1] {
        "" => "",
        "health"=>"health",
        path => {
            println!("{:?}", path);
            "a"
        }
    };

    print!("{:?}", route)
}
