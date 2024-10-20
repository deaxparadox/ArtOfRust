# Alternatives to Using match with Result<T, E>

That’s a lot of match! The match expression is very useful but also very much a primitive. `Closures`, which are used with many of the methods defined on `Result<T, E>`. These methods can be more concise than using match when handling `Result<T, E>` values in your code.


For example, here’s another way to write the same logic, this time using closures and the `unwrap_or_else` method:

```rs
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

