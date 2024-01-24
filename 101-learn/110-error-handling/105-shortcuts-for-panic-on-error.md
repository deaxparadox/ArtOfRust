# Shortcuts for Panic on Error: unwrap and expect

Using match works well enough, but it can be a bit verbose and doesn’t always communicate intent well. `The Result<T, E>` type has many helper methods defined on it to do various, more specific tasks.

----------

#### unwrap method

The unwrap method is a shortcut method implemented just like the match expression. If the Result value is the `Ok` variant, unwrap will return the value inside the `Ok`. If the Result is the `Err` variant, unwrap will call the panic! macro for us. Here is an example of `unwrap` in action:

```rs
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

If we run this code without a *hello.txt* file, we’ll see an error message from the `panic!` call that the `unwrap` method makes:

```bash
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os {
code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:4:49
```

----------


Similarly, the `expect` method lets us also choose the `panic!` error message. Using `expect` instead of `unwrap` and providing good error messages can convery your intent and make tracking down the source of panic easier. The syntax of `expect` looks like this:

```rs
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

We use `expect` in the same way as `unwrap`: to return the file handle or call the `panic!` macro. The error message used by `expect` in its call to `panic!` will be the parameter that we pass to `expect`, rather than the default `panic!` message that `unwrap` uses. Here’s what it looks like:

```bash
thread 'main' panicked at 'hello.txt should be included in this project: Os {
code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:5:10
```

