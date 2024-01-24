# Functions with Return Values

Function can return values t othe code that calls them.

- We declare the return type after an arrow (`->`)
- In Rust, the return value of the function is synonymous with the value of the final experssion in the block of the body of a function.
- You can return early from a function by using the `return` keyword and specifying a value.

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

- The function's return type is specified too, as `-> i32`

```rust
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/functions`
The value of x is: 5
```

- The `5` in `five` is the function's return value, which is why the return type if `i32`. 
- First, the line `let x = five();` shows that we're using the return value of a function to initialize a variable.
- Second, the five function has no parameters and defines the type of the return value, but the body of the function is a lonely `5` with no semicolon because it's an expression whose value we want to return.



Example:

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```


[<<< Statements Experssion](103-Statements-Expression.md) | [Comments >>>](../104-comments.md)