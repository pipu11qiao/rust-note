fn main() {
    let arr = [1, 2, 3];
    let x = &arr[0];

    println!("x is {}", x);
    let mut v = vec![1, 2, 3, 4, 5];

    v[2] = 4;
    let v1 = v;
    let third: &i32 = &v1[2];
    // let t: i32 = v[2];

    // println!("The third element is {}", third);
    // println!("The third element is {}", t);

    // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }
    let v = vec![100,32,57];
for i in &v{
  pringtln!("{}",i)
}
}
