// fn main() {
//     for i in 1..=5 {
//         println!("{}", i)
//     }
// }

// fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2
//     };
 
//     assert_eq!(v, ());
//  }

 fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);
}