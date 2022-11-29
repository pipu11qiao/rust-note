#![allow(dead_code)]
#![allow(unused_variables)]
pub fn main() {
    let arr = [1, 2, 3];

    let mut arr_iter = arr.iter();

    assert_eq!(arr_iter.next(), Some(1).as_ref());
    assert_eq!(arr_iter.next(), Some(2).as_ref());
    assert_eq!(arr_iter.next(), Some(3).as_ref());
    assert_eq!(arr_iter.next(), None);
}
