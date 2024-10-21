# Enabling Recursive Types with Boxes


[⏮️ Using Box<T> to Point Data on Heap](101-using-box-to-point-data-on-heap.md) | [⏸️ Learn](../README.md) | [Deref Trait ⏭️](103-deref-traits.md)

----------

A value of *recursive type* can have another value of the same type as part of itself. Recursive types pose an issue because at compile time Rust needs to known how much space a type takes up. However, the nesting of values of recursive types could theoretical continue infinitely, so Rust can't known how much space the value needs. Because boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition.

As an example of a recursive type, let's explore the *cons list*. This is a data type commonly found in functional programming languages. The cons list type we'll define is straightforward except for the recursion; therefore, the concepts in the example we'll work with will be usefull any time you get into more complex situations involving recursive types.

### Cons List

A *cons list* is a data structure that comes from the Lisp programming language and its dialects and is made up of nested pairs, and is the Lisp version of a linked list.

For example, the pseudocode representation of a cons list containing the list 1, 2, 3 with each pair of parentheses:

```
(1, (2, (3, Nil)))
```

Each item in a cons list contains two elements: 
- The value of the current item and the next item. 
- The last item in the list contains only a value called `Nil` without a next item.

A cons list is produced by recursively calling the `cons` function. The cannonical name to denote the base case of the resursion is `Nil`.


Enum definition for a cons list. Note that this code won’t compile yet because the `List` type doesn’t have a known size, which we’ll demonstrate.

Filename: src/main.rs

```rs
enum List {
    Cons(i32, List),
    Nil,
}

fn main() {}
```

###### Listing 15-2: The first attempt at defining an enum to represent a cons list data structure of `i32` values


Using the `List` type to store the list `1, 2, 3` would look like:

Filename: src/main.rs

```rs
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```
###### Listing 15-3: Using the `List` enum to store the list `1, 2, 3`


- The first `Cons` value holds `1` and another `List` value. This `List` value is another `Cons` value that holds `2` and another `List` value. This `List` value is one more `Cons` value that holds `3` and a `List` value, which is finally `Nil`, the non-recursive variant that signals the end of the list.


If we try to compile the code in Listing 15-3, we get the error shown in Listing 15-4:

```bash
$ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
error[E0072]: recursive type `List` has infinite size
 --> src/main.rs:1:1
  |
1 | enum List {
  | ^^^^^^^^^
2 |     Cons(i32, List),
  |               ---- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +

For more information about this error, try `rustc --explain E0072`.
error: could not compile `cons-list` due to previous error

```

###### Listing 15-4: The error we get when attempting to define a recursive enum

The error shows this type “has infinite size.” The reason is that we’ve defined `List` with a variant that is recursive: it holds another value of itself directly. As a result, Rust can’t figure out how much space it needs to store a `List` value. Let’s break down why we get this error.


### Computing the Size of a Non-Recusive Type


```rs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

To determine how much space to allocate for a `Message` value, Rust goes through each of the variants to see which variant needs the most space. Rust sees that `Message::Quit` doesn’t need any space, `Message::Move` needs enough space to store two `i32` values, and so forth. Because only one variant will be used, the most space a Message value will need is the space it would take to store the largest of its variants.


Contrast this with what happens when Rust tries to determine how much space a recursive type like the `List` enum. The compiler starts by looking at the `Cons` variant, which holds a value of type `i32` and a value of type `List`. Therefore, `Cons` needs an amount of space equal to the size of an `i32` plus the size of a `List`. To figure out how much memory the `List` type needs, the compiler looks at the variants, starting with the `Cons` variant. The `Cons` variant holds a value of type i32 and a value of type `List`, and this process continues infinitely


### Using Box&lt;T&gt; to Get a Recursive Type with a Known Size

Because Rust can't figure out how much space to allocate for recursively defined types, the compiler gives an error with this helpful suggestion:

```
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +
```

----------

In this suggestion, “indirection” means that instead of storing a value directly, we should change the data structure to store the value indirectly by storing a pointer to the value instead.

Because a `Box<T>` is a pointer, Rust always knowns how much space a `Box<T>` needs: a pointer's size doesn't change based on the amount of data it's pointing to. This means we can put a `Box<T>` inside the `Cons` variant instead of another `List` value directly. The `Box<T>` will point to the next `List` value that will be on the heap rather than inside the `Cons` variant. Conceptually, we still have a list, created with lists holding other lists, but this implementation is now more like placing the items next to one another rather than inside one another.


We can change the definition of the `List` enum in [Listing 15-2](#listing-15-2-the-first-attempt-at-defining-an-enum-to-represent-a-cons-list-data-structure-of-i32-values) and the usage of the `List` in [Listing 15-3](#listing-15-3-using-the-list-enum-to-store-the-list-1-2-3) to the code in Listing 15-5, which will compile:

Filename: src/main.rs

```rs
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

###### Listing 15-5: Definition of `List` that uses `Box<T>` in order to have a known size

The `Cons` variant needs the size of an `i32` plus the space to store the box's pointer data. The `Nil` variant stores no values, so it needs less space than the `Cons` variant. We now know that any `List` value will take up the size of an `i32` plus the size of a box’s pointer data. By using a box, we’ve broken the infinite, recursive chain, so the compiler can figure out the size it needs to store a `List` value.

----------

**The `Box<T>` type is a smart pointer because it implements the `Deref` trait, which allows `Box<T>` values to be treated like references. When a `Box<T>` value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the `Drop` trait implementation**

----------


[⏮️ Using Box<T> to Point Data on Heap](101-using-box-to-point-data-on-heap.md) | [⏸️ Learn](../README.md) | [Deref Trait ⏭️](103-deref-traits.md)
