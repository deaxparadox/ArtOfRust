# Statements and Expresions

- **Statements** are instructions that perform some action and do not return a value.
- **Expressions** evaluate to a resultant value. Let’s look at some examples.

Creating a variable and assigning a value to it with the let keyword is a statement.

```rust
fn main() {
    let y = 6;
}
```

- Function definitions are also statements.
- Statements do not return values.
- Expressions evaluate to a value.

A new scope block created with curly brackets is in experssion:

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

- The expression:

```rust
{
    let x = 3;
    x + 1
}
```

- `x + 1` line doesn't have a semicolon at the end. 
- Expressions do not include ending semicolons. 
- If you add a semicolo nto the end of an expression, you turn it into a statement, and it will then not return a value.  

[<<< Parameters](102-Parameters.md) | [Return values >>>](104-functions-return-values.md)