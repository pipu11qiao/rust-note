// 修复下面代码的错误并尽可能少的修改
/*
fn main() {

    let x: i32 = 3; // 未初始化，但被使用
    let _y: i32; // 未初始化，也未被使用
    println!("x is equal to {}", x);
}
// 完形填空，让代码编译
fn main() {
    let mut __ = 1;
    __ += 2;

    println!("x = {}", __);
}

// 修复下面代码的错误并使用尽可能少的改变
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("x 的值是 {}, y 的值是 {}", x, y);
    }
    println!("x 的值是 {}", x);
}

// 修复错误
fn main() {
    println!("{}, world", define_x());
}

fn define_x() -> String {
    let x = String::from("hello");
    x
}


fn main() {
    let mut x: i32 = 1;
    x = 7;
    // 遮蔽且再次绑定
    x += 3;

    let x = x;

    let y = 4;
    // 遮蔽
    let y = "I can also be bound to text!";
}


#[allow(unused_variables)]
fn main() {
    let x = 1;
}

*/
// 修复下面代码的错误并尽可能少的修改
#[allow(unused_mut)]
fn main() {
    let (mut x, mut y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
}
