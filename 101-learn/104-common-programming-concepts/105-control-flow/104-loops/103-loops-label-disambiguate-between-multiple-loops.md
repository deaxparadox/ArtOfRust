# Loop Labels to Disambiguate Between Multiple Loops

- If you have loops within loops, `break` and `continue` apply to the innermost loop at that point.
- You can specify the *loop label* on a loop that you can then use with `break` or `continue` to specify that those keywords apply t othe labeled loop instead of the innermost loop.
- Loop labels must begin with a single quote.

```rust
pub fn main() {
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}");
}
```

The outer lop has the labe `'counting_up`, and it will count up from 0 to 2. 

The inner loop without a label counts down from 10 to 9. The first `break` that doesn't specify a label will exit the inner loop only. 

The `break 'counting_up;` statement will exit the outer loop. This code prints:

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.58s
     Running `target/debug/loops`
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2

```


----------

Here's an full example using `loop label`:

```rs

fn return_from_loop() -> String {
    let mut counter = 0;
    
    'stop_anywhere: loop {
        counter += 1;

        if counter == 10 {
            break 'stop_anywhere;
        }
    };
    let result = counter;
    return format!("The result is {}", result);
}

fn main() {

    let message = return_from_loop();
    println!("{}", message);

}

```

output: 

```bash
The result is 10
```


[<<< Returning values from loops](102-returning-values-from-loops.md) | [Conditional loops with while >>>](104-conitional-loops-with-while.md)