#![allow(dead_code)]
#![allow(unused_variables)]
use std::rc::Rc;

// pub fn main() {
//     let a = Rc::new(String::from("hello world"));
//     let b = Rc::clone(&a);

//     assert_eq!(2, Rc::strong_count(&a));
//     assert_eq!(Rc::strong_count(&b), Rc::strong_count(&a));
// }

struct Owner {
    name: String,
}
struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}
pub fn main(){
    let gadget_owner = Rc::new(Owner{
      name: "Gadget man".to_string(),
    });

    let gadget1 = Gadget{
      id:1,
      owner:Rc::clone(&gadget_owner),
    };
    let gadget2 = Gadget{
      id:2,
      owner:Rc::clone(&gadget_owner),
    };

    drop(gadget_owner);
    
    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);
}


