# Catch-all Patterns and the _ Placeholder


## Placeholder variable 

Imagine we’re implementing a game where, if you roll a 3 on a dice roll, your player doesn’t move, but instead gets a new fancy hat. If you roll a 7, your player loses a fancy hat. For all other values, your player moves that number of spaces on the game board

- Here’s a `match` that implements that logic, with the result of the dice roll hardcoded rather than a random value, and all other logic represented by functions without bodies because actually implementing them is out of scope for this example:

```rs
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```

- For the first two arms, the patterns are the literal values `3` and `7`. 
- For the last arm that covers every other possible value, the pattern is the variable we’ve chosen to name `other`. 
- The code that runs for the `other` arm uses the variable by passing it to the `move_player` function


## Placeholder _

Rust also has a pattern we can use when we want a catch-all but don’t want to use the value in the catch-all pattern: `_` is a special pattern that matches any value and does not bind to that value. This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.

We no longer need to use the catch-all value, so we can change our code to use `_` instead of the variable named `other`:

```rs
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
```

## Nothing happen for placeholder

we’ll change the rules of the game one more time so that nothing else happens on your turn if you roll anything other than a 3 or a 7

```rs
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
```

- So, we have replace `reroll()` with `()`. If we have any other number than 3 and 7, nothing will happen.