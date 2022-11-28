fn main() {
    // let condition = true;
    // let num = if condition { 5 } else { 4 };
    // println!("{}", num);
    // let arr = [5,4,3,2,1];
    let mut arr = [
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
        "e".to_string(),
    ];
    // for item in arr {
    //     println!("item is {}", item);
    // }
    for item in  arr.iter_mut(){
        println!("item is {}", item);
    }
    println!("arr is {:?} ", arr);
}
