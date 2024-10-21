pub mod cons {
    
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use crate::smart_pointers::cons::List::{Cons, Nil};

    fn cons_list()  {
        let list = Cons(
            1, Box::new(Cons(
                2, Box::new(Cons(3, Box::new(Nil)))
            ))
        );
    }
}

pub mod deref {
    pub fn reg_ref() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    pub fn box_ref() {
        let x: i32 = 5;
        let y: Box<i32> = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);

        println!("Comparing value and variable: {}", 5 == x);
        println!("Comparing value and box ref: {}", 5 == *y);
    }


    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
}