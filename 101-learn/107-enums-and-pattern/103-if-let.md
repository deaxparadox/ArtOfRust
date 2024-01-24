# Consice Control Flow with `if-let`

## Regular match

The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle values that match one pattern while ignoring the rest.

Consider the program that matches on an `Option<u8>` value in the `config_max` varaible but only wants to execute code if the value is the `Some` variant:

```rs
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

```

- If the value is `Some`, we print out the value in the `Some` variant by binding the value to the variable `max` in the pattern. 
- We don’t want to do anything with the `None` value. 
- To satisfy the `match` expression, we have to add `_ => ()` after processing just one variant, which is annoying boilerplate code to add.


## if-let

Instead, we could write this in a shorter way using `if let`. The following code behaves the same as the `match`:

```rs
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

```

We can include an `else` with an `if let`. 

-  The block of code that goes with the `else` is the same as the block of code that would go with the `_` case in the `match` expression that is equivalent to the `if let` and `else`.

```rs
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
```

or 

```rs
enum Numbers {
    One(i32),
    Two(i32),
    Three(i32)
}
fn main() {
    let one = Numbers::One(1);

    if let Numbers::One(value) = one {
        println!("Everything Ok, {}", value);
    }
    exit(0);
}

```