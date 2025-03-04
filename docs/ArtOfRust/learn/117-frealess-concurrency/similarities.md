# Similarties between `RefCell<T>/Rc<T>` and `Mutex<T>/Arc<T>`

Similar as the `Cell` family does, the `Mutex<T>` provides interior mutability, as we did in this [example](atomic-reference-counting-wih-arc.md#listing-16-15-using-an-arct-to-wrap-the-mutext-to-be-able-to-share-ownership-across-multiple-threads), the `counter` is immutable but we could get a mutable reference to the value inside it. 

`RefCell<T>` allow us to mutate contents inside an `Rc<T>`, we use `Mutex<T>` to mutate contents inside an `Arc<T>`.

