
## Handling Multiple Conditions with `else if`

You can use multiple conditions by combining `if` and `else` in an  `else if` expression. For example:

```rust
fn main() {
        println!("Guess the number!");
    
        loop {
            println!("\nPlease input your guess: ");
    
            let mut guess = String::new();
        
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess = guess.trim().parse::<i32>().expect("Please type a number!");

            if guess == 0 {
                println!("You have entered 0")
            } else if guess > 0 && guess <= 5 {
                println!("Number lie between 1 and 5")
            } else {
                println!("Unknown number!");
                break;
            }
        };
}
```

```bash
Guess the number!

Please input your guess:
0
You have entered 0

Please input your guess:
3
Number lie between 1 and 5

Please input your guess:
12
Unknown number!
```

- As you can we pass an input an to the program it convert the input in `i32`, after that it compare value using `if-else` and `else-if` expression.


[<<< if expression](101-if-expressions.md) | [using if in let statement >>>](103-using-if-in-let-statement.md)