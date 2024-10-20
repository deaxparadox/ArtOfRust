# Conditional Loops with while

A program will often need to evaluate a condition within a loop.

- While the condition is `true`, the loops runs.
- When the condition ceases to be `true`, the program calls `break`, stopping the loop.

- `while` to loop the program three times, counting down each time, and them, after the loop print a message and exit.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

###### Listing 3-3: Using a while loop to run code while a condition holds true

This construct eliminates a lot of nesting that would be necessary if you used `loop`, `if`, `else`, and `break`, and it’s clearer. While a condition evaluates to `true`, the code runs; otherwise, it exits the `loop`.

```rs
3!
2!
1!
LIFTOFF!!!
```

[Loops label disambiguate between multiple loops](103-loops-label-disambiguate-between-multiple-loops.md) | [Looping through a collection with for](105-looping-through-a-collection-with-for.md)