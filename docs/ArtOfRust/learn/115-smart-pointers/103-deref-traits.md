# The deref traits



[⏮️ Enabling recursive type](102-enabling-recursive-types-with-boxes.md) | [⏸️ Learn](../README.md) | [Deref Trait ⏭️](103-deref-traits.md)

----------


### Introduction

Implementing the `Deref` trait allows you to customize the behavior of  the *dereference operator* `*`. By implementing the `Deref` in such as way that a smart pointer can be treated like a regular reference, you can write code that operates on references and use that code with smart pointers too.


<!-- ### To do

- [Looking at regular reference](https://)
- [Define a custom `Box<T>`](https://)
- [deref coercion](https://) -->

### Following the Pointer to the Value

A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a value stored somewhere else. We are going to createing a reference to a `i32` vaue and then use the dereference operator to follow the reference to the value.

Filename: src/main.rs

```rs
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```
##### Listing 15-6: Using the dereference operator to follow a reference to an `i32` value

The variable `x` holds an `i32` value `5`. We set `y` equal to a reference to `x`. We can assert that `x` is equal to `5`. However, if we want to make an assertion about the value in `y`, we have to use `*y` to follow the reference to the value it’s pointing to (hence dereference) so the compiler can compare the actual value. Once we dereference `y`, we have access to the integer value `y` is pointing to that we can compare with `5`.

If we tried to write `assert_eq!(5, y);` instead, we would get this compilation error.

```bash
$ cargo run
   Compiling deref-example v0.1.0 (file:///projects/deref-example)
error[E0277]: can't compare `{integer}` with `&{integer}`
 --> src/main.rs:6:5
  |
6 |     assert_eq!(5, y);
  |     ^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
  |
  = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `deref-example` (bin "deref-example") due to 1 previous error

```

Comparing a number and a reference to a number isn't allowed because they're different types. We must use the dereference operator to follow the reference to the value it's pointing to.

### Using Box&lt;t&gt; like a reference

We can write the code in [Listing 15-6](#listing-15-6-using-the-dereference-operator-to-follow-a-reference-to-an-i32-value) to use a `Box<T>` instead of a reference. The dereference operator used on the `Box<T>` in Listing 15-7 functions in the same way as the dereference operator used on the reference in Listing 15-6.

Filename: src/main.rs

```rs
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

##### Listing 15-7: Using the dereference operator on a `Box<i32>`

The main difference between Listing 15-7 and Listing 15-6 is that here we set `y` to be an instance of a `Box<T>` pointing to a copied value of `x` rather than a reference pointing to the value of `x`. In the last assertion, we can use the dereference operator to follow the pointer of the `Box<T>` in the same way that we did when `y` was a reference.

### Defining out own smart pointer

Let's build a smart poitner similar to the `Box<T>` type provided byte standard library to experience how smart pointers behave differently from references by default.

The `Box<T>` type is ultimately defined as a tuple struct with one element, so Listing 15-8 defines a `MyBox<T>` type in the same way. We'll also define a `new` function to match the `new` function defined on the `Box<T>`.

Filename: src/main.rs

```rs
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

##### Listing 15-8: Defining a `MyBox<T>` type

We define a struct named `MyBox` and declare a generic parameter `T`, because we want our type to hold values of any type. The `MyBox` type is a tuple struct with one element of type `T`. The `MyBox::new` function takes one parameters of type `T` and returns a `MyBox` instance that holds the value passed in.

Let’s try adding the `main` function in Listing 15-7 to Listing 15-8 and changing it to use the `MyBox<T>` type we’ve defined instead of `Box<T>`. The code in Listing 15-9 won’t compile because Rust doesn’t know how to dereference MyBox.

Filename: src/main.rs

```rs
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

##### Listing 15-9: Attempting to use `MyBox<T>` in the same way we used references and `Box<T>`

Here’s the resulting compilation error:


```rs
$ cargo run
   Compiling deref-example v0.1.0 (file:///projects/deref-example)
error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
  --> src/main.rs:14:19
   |
14 |     assert_eq!(5, *y);
   |                   ^^

For more information about this error, try `rustc --explain E0614`.
error: could not compile `deref-example` (bin "deref-example") due to 1 previous error

```

Our `MyBox<T>` type can’t be dereferenced because we haven’t implemented that ability on our type. To enable dereferencing with the `*` operator, we implement the `Deref` trait.


### Implementing the `Deref` trait

To implement a trait, we need to provide implementations for the trait's required methods. The `Deref` trait, provided by the standard library, requires us to implement one method named `deref` that borrows `self` and returns a reference to the inner data. Listing 15-10 contains an implementation of `Deref` to add to the definition of `MyBox`:


```rust
pub trait Deref {
    type Target: ?Sized;

    // Required method
    fn deref(&self) -> &Self::Target;
}
```

Used for immutable dereferencing operations, like `*v`.

In addition to being used for explicit derefrencing operations with the (unary) `*` operator in immutable contexts, `Deref` is also used implicitly by the compiler in many circumtances. The mechanism is called `"Deref coercion"`. 

In mutable contexts `DerefMut` is used and mutable deref coercion similarly occurs.


----------

**Warning**: Deref coercion is a powerful language feature which has far-reaching implications for each type that implements `Deref`. The compiler will silently insert calls to `Deref::deref`. For this reason, one should be carefull about implementing `Deref` and only do so when deref coercion is desirable.


----------


Type that implement `Deref` or `DerefMut` are often called "smart pointers" and the mechanism of deref coercion has been specifically designed to facilitate the pointer-like behaviour that name suggests.

Often, the purple of a "smart pointer" type is to change the ownership sementics of a contained value (for example, `Rc` or `Cow`) or the storage sementics of a contained value (for example, `Box`).