# Functions

The `main` function, which is the entry point of many programs.

- The fn keyword, which allows you to declare new functions.
- Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.


```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/functions`
Hello, world!
Another function.
```

[<<< Functions](README.md) | [Parameters >>>](102-Parameters.md)


