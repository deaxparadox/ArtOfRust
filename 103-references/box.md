# Box `std::boxed`

The `Box<T>` type for heap allocation.

`Box<T>`, casually referred to as a ‘box’, provides the simplest form of heap allocation in Rust. Boxes provide ownership for this allocation, and drop their contents when they go out of scope. Boxes also ensure that they never allocate more than isize::MAX bytes.

A type that uniquely owns a heap allocation of type `T`.

----------

Allocate memory on the heap and them places `x` into it.

This  doesn't actually allocated if `T` is zero-sized.

```rs
let five = Box::new(5);
```

Here we have used `new` method of allocated memory.

----------

### Examples

Move a value from stack to the heap by creating a `Box`:

```rs
    let val: u8 = 5;
    let boxed: Box<u8> = Box::new(val);
    println!("{boxed}");
```

Move a value from a `Box` back to the stack by `dereferencing`:

```rs

    let boxed: Box<u8> = Box::new(5);
    let val = *boxed;
    println!("{boxed}");

```

Creating a recursive data structure:

```rs
use std::process::exit;

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{list:?}");

    exit(0);
}
```

This will print `Cons(1, Cons(2, Nil))`.

----------

Recursive structures must be boxed, because if the definition of `Cons` looked like this:

```rs
Cons(T, List<T>),
```
It wouldn’t work. This is because the size of a `List` depends on how many elements are in the list, and so we don’t know how much memory to allocate for a `Cons`. By introducing a `Box<T>`, which has a defined size, we know how big `Cons` needs to be.