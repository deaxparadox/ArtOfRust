# Generating a Secret Number

We need to generate a secret number that the user will try to guess. 

## Using a Crate to Get more Functionality

Remeber that a crate is a collection of Rust source code files.

- The project we've been building is a *binary crate*, which is an executable.
- The `rand` crate is a *library crate*, which contains code that is intended to be used in other programs and can't be executed on its own.

- We need inlcude the `rand` depenency to *Cargo.toml*, so that we can used `rand` crate.

```rs
// Cargo.toml

[dependencies]
rand = "0.8.5"
```

- Now, without changing any of the code, let's bulid the project.

```bash
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
  Downloaded libc v0.2.127
  Downloaded getrandom v0.2.7
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.16
  Downloaded rand_chacha v0.3.1
  Downloaded rand_core v0.6.3
   Compiling libc v0.2.127
   Compiling getrandom v0.2.7
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.16
   Compiling rand_core v0.6.3
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s

```

## Generating a Random Number

Let's start using `rand` to generate a number to guess. The next stop is to update *src/main.rs*:

```rs
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

- The `Rng` trait defines methods that random number generator implements, and this trait must be in scope for to use those methods.