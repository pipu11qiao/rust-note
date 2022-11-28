// mod struce_borrowed_error;
#[derive(Debug)]
struct Book {
    name: String,
}
impl Book {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    let mut book = Book {
        name: "hello".to_string(),
    };


    let name = book.get_name();


    println!("{:?}",&book)
}
