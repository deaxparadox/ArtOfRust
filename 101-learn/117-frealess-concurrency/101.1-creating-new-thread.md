# Creating a New Thread with spawn

To create a new thread:

- we call the `thread::spawn` function and pass it a closure containing the code we want to run in the new thread.

- The example prints some text from a main thread and other text from the new thread:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

- **Note that when the main thread of a Rust program completes, all spawned threads are shut down,
 whether or not they have finshed running.

```bash
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
```


- The calls to `thread::sleep` force a thread to stop its execution for a short duration, allowing a different thread to run.
- The threads will probably take turns, but that isn't guaranteed: it depends on how your operating system schedues the threads.
- In this run, the main thread printed first, even though the print statement from the spawned thread appears first in the code.
- And even though we told the spawned thread to print until `1` is 9, it only got to 5 before the main thread shut down.

[<<< Using threads](101-using-threads.md)... [Waiting for all threads >>> ](101.2-waiting-for-all-threads.md)
