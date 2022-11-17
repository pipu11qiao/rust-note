fn base_demo() {
    fn main() {
        println!("Hello, world!");
        let width1 = 30;
        let height1 = 50;
        println!(
            "The area of the rectangle is {} square pixels.",
            area(width1, height1)
        );
    }

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
}
fn tuple_demo() {
    fn main() {
        let rect1 = (30, 50);
        println!(
            "The area of the rectangle is {} square pixels.",
            area(rect1)
        );
    }

    fn area(dimentions: (u32, u32)) -> u32 {
        dimentions.0 * dimentions.1
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn rect_demo() {
    fn main() {
        let rect1 = Rectangle {
            width: 30,
            height: 30,
        };
        println!("rect1 is {:?}", rect1);
        println!("rect1 is {:#?}", rect1);

        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
        );
    }
    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}
fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
    let x = 5;
    dbg!(x);
}
