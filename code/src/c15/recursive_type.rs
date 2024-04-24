

enum List {
    Cons(i32, Box<List>),
    Nil
}

use crate::c15::recursive_type::List::{Cons, Nil};

fn main() {
    let list = List::Cons(
        1, Box::new(Cons(
            2, Box::new(Cons(
                3, Box::new(Nil)
            ))
        ))
    );
}