# Using Box&lt;T&gt; to Point to Data on the Heap


[⏮️ Smart Pointes](README.md) | [⏸️ Learn](../README.md) | [Enabling Recursive Types with Boxes ⏭️ ](102-enabling-recursive-types-with-boxes.md)

----------

### Table of contents

- [Introduction](#introduction)
- [Usecases of Box&lt;T&gt;](#usecases-of-boxt)
----------


### Introduction

The most straightforward smart pointer is a *box*, whose type is written `Box<T>`. Boxes allows you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data.

**Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities**.


Boxes don't have performance overhead, other than storing their data on the heap instead of on the stack. But they don't have many extra capabilities either. 

### Usecases of Box&lt;T&gt;

You'll use them most in these situations:

- When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
- When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
- When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type


### Using a Box<T> to Store Data on the Heap

Listing 15-1 shows how to use a box to store an `i32` value on the heap:

Filename: src/main.rs

```rs
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

###### Listing 15-1: Storing an `i32` value on the heap using a box

- We define the variable `b` to have the value of a `Box` that points to the value `5`, which is allocated on the heap. 
- This program will print `b=5;` in this case, we can access the data in the box similar to how we would if this data were on the stack. 
- Just like any owned value, when a box goes out of scope, as `b` does at the end of `main`, it will be deallocated. This deallocation happens both for the box (stored on the stack) and the data it points to (stored on the heap).



[<<< Smart Pointers](README.md) | [Home](../README.md) | [Recursive Types >>>](102-enabling-recursive-types-with-boxes.md)