# Returning Values from Loops

One of the ues of a `loop` is to retury an operation you known might fail, such as checking whether a thread has completed its job.

You might also need to pass the result of that operation out of the loop to the rest of your code. To do this, you can add the value you want returned after the `break` expression you use to stop the loop; that value will be returned out of the loop so you can use it in your code:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

- Before the `loop`, we declare a variable named `counter` and initialize it to 0.
- Then we declare a variable named `result` to hold the value returned from the loop.
- On every iteration of the loop, we add `1` to the counter variable, and then check whether the `counter` is equal to `10`.
- When it is, we use the `break` keyword with the value `counter * 2`. 
- After the loop, we use a semicolon to end the statement that assigns the value to `result`. Finally, we print the value in `result`, which in this case is `20`.

----------


Here is the completed example:

```rs
fn return_from_loop() -> String {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    return format!("The result is {}", result);
}

fn main() {

    let message = return_from_loop();
    println!("{}", message);

}
```

Output: 

```bash
The result is 20
```

[<<< Repeating code with loop](101-repeating-code-with-loop.md) | [Loops Labels to disambiguate Between Mulitple Loops >>>](103-loops-label-disambiguate-between-multiple-loops.md)