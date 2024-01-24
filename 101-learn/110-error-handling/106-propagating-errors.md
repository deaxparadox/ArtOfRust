# Propagating Errors

When a function’s implementation calls something that might fail, instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do. This is known as *propagating* the error and gives more control to the calling code, where there might be more information or logic that dictates how the error should be handled than what you have available in the context of your code.

For example, a function that reads a username from a file. If the file doesn't exist or can't be read, this function will return those errors to the code that called the function.

```rs
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

```

- The return type of the function first: `Result<String, io::Error>`. This means the function is returning a value of the type `Result<T, E>` where the generic parameter `T` has been filled in with the concrete type `String`, and the generic type `E` has been filled in with the concrete type `io::Error`.

If this function succeeds without any problems, the code that calls this function will receive an `Ok` value that holds a `String`—the username that this function read from the file. If this function encounters any problems, the calling code will receive an `Err` value that holds an instance of `io::Error` that contains more information about what the problems were. We chose `io::Error` as the return type of this function because that happens to be the type of the error value returned from both of the operations we’re calling in this function’s body that might fail: the `File::open` function and the `read_to_string` method.

The body of the function starts by calling the `File::open` function. Then we handle the `Result` value with a `match`. If `File::open` succeeds, the file handle in the pattern variable file becomes the value in the mutable variable `username_file` and the function continues. In the `Err` case, instead of calling `panic!`, we use the return keyword to return early out of the function entirely and pass the error value from `File::open`, now in the pattern variable `e`, back to the calling code as this function’s error value.

So if we have a file handle in `username_file`, the function then creates a new `String` in variable `username` and calls the `read_to_string` method on the file handle in `username_file` to read the contents of the file into `username`. The `read_to_string` method also returns a `Result` because it might fail, even though `File::open` succeeded. So we need another `match` to handle that `Result:` if `read_to_string` succeeds, then our function has succeeded, and we return the `username` from the file that’s now in `username` wrapped in an `Ok`. If `read_to_string` fails, we return the error value in the same way that we returned the error value in the `match` that handled the return value of `File::open`.

The code that calls this code will then handle getting either an `Ok` value that contains a `username` or an `Err` value that contains an `io::Error`. It’s up to the calling code to decide what to do with those values. If the calling code gets an `Err` value, it could call `panic!` and crash the program, use a default username, or look up the username from somewhere other than a file, for example. We don’t have enough information on what the calling code is actually trying to do, so we propagate all the success or error information upward for it to handle appropriately.

----------


## A Short for Propagating Errors: the ? Operator

An implementation of `read_username_from_file` that has the same functionality, but this implementation uses the `?` operator.

```rs
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

The `?` placed after a `Result` value is defined to work in almost the same way as the match expressions we defined to handle the `Result` values. If the value of the `Result` is an `Ok`, the value inside the `Ok` will get returned from this expression, and the program will continue. If the value is an `Err`, the `Err` will be returned from the whole function as if we had used the `return` keyword so the error value gets propagated to the calling code.

----------

There is a difference between what the `match` expression does and what the `?` operator does: error values that have the `?` operator called on them go through the from function, defined in the `From` trait in the standard library, which is used to convert values from one type into another. When the `?` operator calls the from function, the error type received is converted into the error type defined in the `return` type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.

----------

For example, we could change the `read_username_from_file` function to return a custom error type named `OurError` that we define. If we also define `impl From<io::Error>` for `OurError` to construct an instance of `OurError` from an `io::Error`, then the `?` operator calls in the body of `read_username_from_file` will call from and convert the error types without needing to add any more code to the function.

The `?` at the end of the `File::open` call will return the value inside an `Ok` to the variable `username_file`. If an error occurs, the `?` operator will return early out of the whole function and give any `Err` value to the calling code. The same thing applies to the `?` at the end of the `read_to_string` call.

The `?` operator eliminates a lot of boilerplate and makes this function’s implementation simpler. We could even shorten this code further by chaining method calls immediately after the `?`, as shown:

```rs
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

We’ve moved the creation of the new `String` in `username` to the beginning of the function. Instead of creating a variable `username_file`, we’ve chained the call to `read_to_string` directly onto the result of `File::open("hello.txt")?`. We still have a `?` at the end of the `read_to_string` call, and we still return an `Ok` value containing `username` when both `File::open` and `read_to_string` succeed rather than returning errors.


To make this even shorter usign `fs::read_to_string`

```rs
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

Reading a file into a string is a fairly common operation, so the standard library provides the convenient `fs::read_to_string` function that opens the file, creates a new `String`, reads the contents of the file, puts the contents into that `String`, and returns it. Of course, using `fs::read_to_string` doesn’t give us the opportunity to explain all the error handling, so we did it the longer way first.


----------


### Where The "?" Operator can be used

The `?` operator can only be used in functions whose return type is compatible with the value the `?` is used on. This is because `?` operator is defined to perform en early return of value of the function, in the same mamaber as the match expression, the `match` was using a `Result` value, and the early return arm returned an `Err(e)` value. The return tpe of the function has to be a `Result` so that it's compatible with this `return`.

**We’re only allowed to use the `?` operator in a function that returns Result, Option, or another type that implements FromResidual.**

To fix the error, you have two choices.

1. One choice is to change thereturn type of your function to be compatible with the value you're using the `?` operator on as long as you have no restrictions preventing that. 
2. The other techniques is to use a mtch or one of the `Result<T, E>` method to handlethe `Result<T, E>` in whatevery way is appropriate.

----------

`?` can be used with `Option<T>` values as well. The behavior of the `?` operator when called on an `Option<T>` is similar to its behavior when called on `Result<T, E>`: if the value is `None`, the `None` will be returned early from the function at that point. If the value is `Some`, the value inside the `Some` is the resulting value of the expression and the function continues.

An example of a function that finds the last character of the first line:

```rs
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

This function returns `Option<char>` because it’s possible that there is a character there, but it’s also possible that there isn’t. This code takes the `text` string slice argument and calls the `lines` method on it, which returns an iterator over the lines in the string. Because this function wants to examine the first line, it calls `next` on the iterator to get the first value from the iterator. If text is the empty string, this call to next will return `None`, in which case we use `?` to stop and return `None` from `last_char_of_first_line`. If `text` is not the empty string, `next` will return a `Some` value containing a string slice of the first line in text.

----------

Note that you can use the `?` operator on a `Result` in a function that returns `Result`, and you can use the `?` operator on an `Option` in a function that returns `Option`, but you can’t mix and match. The `?` operator won’t automatically convert a `Result` to an `Option` or vice versa; in those cases, you can use methods like the `ok` method on `Result` or the `ok_or` method on `Option` to do the conversion explicitly.

----------

The `main` function is special because it’s the entry and exit point of executable programs, and there are restrictions on what its return type can be for the programs to behave as expected. All the generally return `()`.

Luckily, main can also return a `Result<(), E>`. In bellow example we’ve changed the return type of main to be `Result<(), Box<dyn Error>>` and added a return value `Ok(())` to the end. This code will now compile:

```rs
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```

The `Box<dyn Error>` type is a *trait object*. For now, you can read `Box<dyn Error>` to mean “any kind of error.” Using `?` on a `Result` value in a main function with the error type `Box<dyn Error>` is allowed, because it allows any `Err` value to be returned early. Even though the body of this main function will only ever return errors of type `std::io::Error`, by specifying `Box<dyn Error>`, this signature will continue to be correct even if more code that returns other errors is added to the body of main.

When a `main` function returns a `Result<(), E>`, the executable will exit with a value of `0` if main returns `Ok(())` and will exit with a nonzero value if main returns an `Err` value.

The `main` function may return any types that implement the `std::process::Termination` `trait`, which contains a function report that returns an `ExitCode`. 