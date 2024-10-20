# Matching with different Errors

We want to take differetnt actions for different failure reasons: if `File::open` failed because the file doesn't exist, we want to create a file and return the handle to the new file. If `File::open` failed for any other reason-for example, because we didn't hage permission to open the file--we still want the code to `panic!`. For this we add an inner `match` expression:

```rs
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```

The type of the value that `File::open` returns inside the `Err` varaint is `io::Error`, which is a struct provided by the standard library. This struct has a method `kind` that we can call to get an `io::Errorkind` value. The enum `io::Errorkind` is provided by the standard library and has variants representing the different kinds of errors that might result from an `io` operation. The variant we want to use is `ErrorKind::NotFound`, which indicates the file we're trying to open doesn't exist yet. So we match on `greeting_file_result`, but we also have an inner match on `error.kind()`.


The condition we want to check in the inner match is whether the value returned by `error.kind()` is the `NotFound` variant of the `ErrorKind` enum. If it is, we try to create the file with `File::create`. However, because `File::create` could also fail, we need a second arm in the inner match expression. When the file can’t be created, a different error message is printed. The second arm of the outer match stays the same, so the program panics on any error besides the missing ile error.