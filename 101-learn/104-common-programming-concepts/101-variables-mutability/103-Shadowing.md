# Shadowing

- When we say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable.
- In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends. 

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

```bash
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/variables`
The value of x in the inner scope is: 12
The value of x is: 6
```

- This program first binds x to a value of 5. Then it creates a new variable x by repeating let x =, taking the original value and adding 1 so the value of x is then 6. Then, within an inner scope created with the curly brackets, the third let statement also shadows x and creates a new variable, multiplying the previous value by 2 to give x a value of 12. When that scope is over, the inner shadowing ends and x returns to being 6

[<<< Contants](102-Contants.md) | [Len functions >>>](104-len-function.md)