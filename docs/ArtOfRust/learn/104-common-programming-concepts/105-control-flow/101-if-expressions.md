# if Expressions

- An `if` expression allows you to branch you code depending on conditions.
- You provide a condition and then state, "If this condition is met, run this block of code. If the condition is not met, do not run this block of code."


```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

- The condition checks whether or not the variable `number` has a value less than 5. 
- We place the block of code to exectue if the condition is `true` immediately after the condition inside curly brackets.
- if condition is `false` then code block after `else` expression is executed.

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
condition was true
```


It's also worth noting that the condition in this code *must* be a `bool`.

```rust
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

- The `if` condition evaluates to a value of `3` this time, and Rust throws an error:

```rust
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

- The error indicates that Rust expected a `bool` but got a integer. 
- Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean. You must be explicit and always provide `if` with a Boolean as its condition.
- If we want the `if` code block to run only when a number is not equal to `0`

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```



[<<< control flow](README.md) | [handling multiple condition](102-handling-multiple-condition.md)