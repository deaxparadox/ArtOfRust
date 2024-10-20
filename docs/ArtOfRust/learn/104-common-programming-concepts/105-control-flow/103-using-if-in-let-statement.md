## Using if in a let Statement


Because `if` is an expression, we can use it on the right side of a `let` statement to assign the outcome to a variable.

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

- The `number` variable will be bound to a value based on the outcome of the `if` expression.

```output
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/branches`
The value of number is: 5
```

- the values that have the potential to be results from each arm of the `if` must be the same type.
- `let number = if condition { 5 } else { 6 };` value in both `if` and `else` block must be of same type.




[<<< Handling multiple conditions](102-handling-multiple-condition.md) | [Loops >>>](104-loops/README.md)