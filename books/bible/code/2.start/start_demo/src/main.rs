use std::process::Command;

fn main() {

    let _child = Command::new("kill")
        .args(["-9", &format!("{}", "12299")])
        .spawn();
    match _child {
        Ok(f) => "",
        Err(_e) => {
            print!("{}", _e);
            ""
        }
    };
}
