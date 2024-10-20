# Repeating Code with `loop`

The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.


```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

When we run this program, we'll see `again!` printed over and over continously until we stop the programming manually.

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

- Press control+c to break the loop.

- We also use `continue` for to tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration.

[<<< Loops](README.md) | [Returning values from loops >>>](102-returning-values-from-loops.md)