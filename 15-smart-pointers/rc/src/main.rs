use std::rc::Rc;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(1, Rc::new(Nil)));
    let b = Cons(2, Rc::clone(&a));
    let c = Cons(3, Rc::clone(&a));

    println!("{:?} {:?}", b, c);
}
