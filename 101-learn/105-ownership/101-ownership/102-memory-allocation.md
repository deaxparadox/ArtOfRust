# Memory and Allocation

**Why can `String` be mutated but literals cannot? The difference is in how these two types deal with memory.**


In the case of string literal, we known the contents at compile tiem, so the text is hardcoded directly into the final executable.

- This is why string literals are fast and efficient.
- But these properties only come from the string literal's immutability

With the `String` type, in order to support a mutable, growable piece of text, we need to allocated an amount of memory on the heap, unknown at compile time:

- The memory must be requested from the memory allocator at runtime.
- We need a way of returning this memory to the allocator when we're done with our `String`.

That first part is done by us: when we call `String::from`, its implementation requests the memory it needs. This is pretty much universal in programming languages.

Howevery, the second part is different: in Rust memory is automatically returned once the variable that owns it goes out of scope.

Here’s a version of our scope example from Listing 4-1 using a `String` instead of a string literal:

```rs
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no
                                    // longer valid

```

- There is a natural point at which we can return the memory our `String` needs to the allocator: when `s` goes out of scope.
- When a variable goes out of scope, Rust calls a special function for us. This function is called `drop`, and it's where the author of `String` can put the code to return the memory.
- Rust calls `drop` automatically at the closing curly bracket.


## Variables and Data Interacting with Move

Multiple variables can interact witht the same data in different ways in Rust. Let's look at an example using an integer.

```rs
let x = 5;
let y = x;
```

###### Listing 4-2: Assigning the integer value of variable x to y


- We can probably guess what this is doing: “bind the value `5` to `x`; then make a copy of the value in `x` and bind it to `y`.” 
- We now have two variables, `x` and `y`, and both equal `5`. 
- This is indeed what is happening, because integers are simple values with a known, fixed size, and these two `5` values are pushed onto the stack.

Now let's look at the `String` version:

```rs
let s1 = String::from("hello");
let s2 = s1;
```

- A `String` is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity. 
- This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.

![Image 1](trpl04-01.svg)

- The length is how much memory, in bytes, the contents of the String are currently using. 
- The capacity is the total amount of memory, in bytes, that the String has received from the allocator. 
- The difference between length and capacity matters, but not in this context, so for now, it’s fine to ignore the capacity.

When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. 

- We do not copy the data on the heap that the pointer refers to.
- the data representation in memory looks like

![Image 2](trpl04-02.svg)

The representation below, which is what memory would look like if Rust instead copied the heap data as well.

- If Rust did this, the operation `s2 = s1` could be very expensive in terms of runtime performance if the data on the heap were large.

![Image 3](trpl04-03.svg)


When s2 and s1 go out of scope, they will both try to free the same memory. 

- This is known as a ***double free*** error and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

```rs
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
```

- To ensure memory safety, after the line `let s2 = s1;`, Rust considers `s1` as no longer valid.
- Therefore, Rust doesn’t need to free anything when `s1` goes out of scope. Check out what happens when you try to use `s1` after `s2` is created; it won’t work: 

```bash
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` due to previous error

```

If you've heard the terms *shallow copy* and *deep copy* while working with other languages, the concept the pointer, length, and capacity without copying the data probably sounds like makeing a **shallow copy**.

- But because Rust also invalidates the first variable, instead of being called a **shallow copy**, it’s known as a **move**. In this example, we would say that *s1* was moved into *s2*.

![Image 4](trpl04-04.svg)

- That solves our problem! With only `s2` valid, when it goes out of scope it alone will free the memory, and we’re done.

## Variables and Data Interacting with Clone

If we do want to deeply copy the heap data of the `String`, not just the stack data, we can use a common method called `clone`.

```rs
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

## Stack-Only Data: Copy

```rs
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
```

- But this code seems to contradict what we just learned: we don’t have a call to `clone`, but `x` is still valid and wasn’t moved into `y`.

- The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.

Rust has a special annotation called the `Copy` trait that we can place on types that are stored on the stack, as integers are. 

- If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

Here are some of the types that implement `Copy`:

- All the integer types, such as `u32`.
- The Boolean type, `bool`, with values `true` and `false`.
- All the floating-point types, such as `f64`.
- The character type, `char`.
- Tuples, if they only contain types that also implement `Copy`. For example, `(i32, i32)` implements `Copy`, but `(i32, String)` does not.
