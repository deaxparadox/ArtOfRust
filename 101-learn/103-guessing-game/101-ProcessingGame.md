# Processing a Game

The first part of the guessing game programm will ask for user input, process that input and check that input is in the expected form.

```rs
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

- To obtain user input and then print the result as output, we need to bring the `io` input/output library into scope.

```rs
use std::io;
```

- By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the ***prelude***

- `fn` syntax declares a new function; the parentheses, `()`, indicate there are no parameters; and the curly bracket, `{`, starts the body of the function.

- `println!` is a macro that prints a string to the screen.

```rs
println!("Guess the number!");

println!("Please input your guess.");
```

[>>>](102-StoringValues.md)