# Validating References with Lifetimes

Lifetimes are another kind of generic. Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.

**Reference in Rust has a lifetime, which is the scope for which that reference is valid**. Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred. We must only annotate types when multiple types are possible. In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways. Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

## Preventing Dangling References with Lifetimes

The main aim of lifetimes is to prevent *dangling references*, which cause a program to reference data other than the data it's intended to reference. Consider the program, which has an outer scope and an inner scope.

```rs
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```


The outer scope declares a variable named `r` with no initial value, and the inner scope declares a variable named `x` with the initial value of `5`. Inside the inner scope, we attempt to set the value of `r` as a reference to `x`. Then the inner scope ends, and we attempt to print the value in `r`. This code won’t compile because what the value `r` is referring to has gone out of scope before we try to use it. Here is the error message:

```bash
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `x` does not live long enough
 --> src/main.rs:6:13
  |
6 |         r = &x;
  |             ^^ borrowed value does not live long enough
7 |     }
  |     - `x` dropped here while still borrowed
8 |
9 |     println!("r: {}", r);
  |                       - borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10` due to previous error
```

The variable `x` doesn’t “live long enough.” The reason is that `x` will be out of scope when the inner scope ends on line 7. But `r` is still valid for the outer scope; because its scope is larger, we say that it “lives longer.” If Rust allowed this code to work, `r` would be referencing memory that was deallocated when `x` went out of scope, and anything we tried to do with `r` wouldn’t work correctly. So how does Rust determine that this code is invalid? It uses a borrow checker.

## The Borrow Checker

The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid. Below shows the same code as above but with annotations showing the lifetimes of the variables.

```rs
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

Here, we’ve annotated the lifetime of `r` with `'a` and the lifetime of `x` with `'b`. As you can see, the inner `'b` block is much smaller than the outer `'a` lifetime block. At compile time, Rust compares the size of the two lifetimes and sees that `r` has a lifetime of `'a` but that it refers to memory with `a` lifetime of `'b`. The program is rejected because `'b` is shorter than `'a`: the subject of the reference doesn’t live as long as the reference.

Let's fixes the code so it doesn’t have a dangling reference and compiles without any errors.

```rs
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```

Here, `x` has the lifetime `'b`, which in this case is larger than `'a`. This means `r` can reference `x` because Rust knows that the reference in `r` will always be valid while `x` is valid.

Now that you know where the lifetimes of references are and how Rust analyzes lifetimes to ensure references will always be valid, let’s explore generic lifetimes of parameters and return values in the context of functions.

[generic-lifetimes-in-functions >>>](101-generics-lifetimes-in-functions.md)