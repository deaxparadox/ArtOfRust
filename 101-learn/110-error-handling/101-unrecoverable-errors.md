# Unrecoverable Errors with panic!

Let's try calling `panic!` in a simple program:

```rs
fn main() {
    panic!("crash and burn");
}
```

- When you run the program:

```bash
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- The call to `panic!` causes the error message contained in the last two lines. The first line shows our panic message and the place in our source code where the panic occurred: *src/main.rs:2:5* indicates that it’s the second line, fifth character of our *src/main.rs* file

----------


### Using a panic! Backtrace


Let's look at another example to see what it's like when a `panic!` call comes from a library because of a bug in our code calling the macro directly.

Let have some code that attempts to access an index in a vector beyound the range of valid indexes.

```rs
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

- Here, we're attempting to access the 100th element of our vector (which is at index 99 because indexing starts at zero), but the vector has only 3 elements. In this situation, Rust will panic. 

- Using `[]` is supposed to return an element, but if you pass in invalid index, there's no element that Rust could return here that would be correct.

*In C, attempting to read beyond the end of a data structure is undefined behavior. You might get whatever is at the location in memory that would correspond to that element in the data structure, even though the memory doesn’t belong to that structure. This is called a buffer overread and can lead to security vulnerabilities if an attacker is able to manipulate the index in such a way as to read data they shouldn’t be allowed to that is stored after the data structure.*

```bash
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- This error points at line 4 of our main.rs where we attempt to access index 99. The next note line tells us that we can set the `RUST_BACKTRACE` environment variable to get a backtrace of exactly what happened to cause the error. 
- A *backtrace* is a list of all the functions that have been called to get to this point. 
- Backtraces in Rust work as they do in other languages: the key to reading the backtrace is to start from the top and read until you see files you wrote. That’s the spot where the problem originated. The lines above that spot are code that your code has called; the lines below are code that called your code. These before-and-after lines might include core Rust code, standard library code, or crates that you’re using. Let’s try getting a backtrace by setting the `RUST_BACKTRACE` environment variable to any value except `0`.