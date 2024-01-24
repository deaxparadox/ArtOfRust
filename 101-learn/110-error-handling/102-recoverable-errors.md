# Recoverable Errors with Result

Rust have the `Result` enum, that define to varaiants, `Ok` and `Err`, as follows:

```rs
enum Result<T, E> {
    Ok(T),
    Err(E),
}

```

- There `T` and `E` are generic type parameters. `T` represents the type of the value that will be returned in a success case within the `Ok` variant, and `E` represents the type of the error that will be returned in a failure case within the `Err` variant.

----------


let's call a function that returns a `Result` value because the function could fail. If we try to open a file:

```rs
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
```

- The return type of `File::open` is a `Result<T, E>`. The generic parameter `T` has been filled in by the implementation of `File::open` with the type of the success value, `std::fs::File`, which is a file handle. The type of `E` used in the error value is `std::io::Error`.
- This return type means the call to `File::open` might succeed and return a file handle that we can read from or write to. The function call also might fail: for example, the file might not exist, or we might not have permission to access the file. The `File::open` function needs to have a way to tell us whether it succeeded or failed and at the same time give us either the file handle or error information. This information is exactly what the Result enum conveys.


In the case where `File::open` succeeds, the value in the variable `greeting_file_result` will be an instance of `Ok` that contains a file handle. In the case where it fails, the value in `greeting_file_result` will be an instance of `Err` that contains more information about the kind of error that happened.


We need to add to the code, to take different actions depending on the value `File::open` returns. One way to handle the Result using a basic tool:

```rs
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

Note that, like the `Option` enum, the `Result` enum and its variants have been brought into scope by the *prelude*, so we don’t need to specify `Result::` before the `Ok` and `Err` variants in the match arms.

When the result is `Ok`, this code will return the inner file value out of the `Ok` variant, and we then assign that file handle value to the variable `greeting_file`. After the match, we can use the file handle for reading or writing.

The other arm of the match handles the case where we get an `Err` value from `File::open`. In this example, we’ve chosen to call the `panic!` macro. If there’s no file named `hello.txt` in our current directory and we run this code, we’ll see the following output from the `panic!` macro:

```bash
$ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
    Finished dev [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/error-handling`
thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:23
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```