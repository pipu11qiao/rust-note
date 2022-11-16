fn variables() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
fn scalar_types() {
    let floord: f32 = 2.0 / 3.0;

    println!(" {floord}");
}
fn tuple_demo() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let num1 = tup.0;
    println!("{num1}")
}
fn main() {
    tuple_demo();
}
