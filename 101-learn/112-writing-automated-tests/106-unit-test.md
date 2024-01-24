# Unit Tests

The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn't working as expected.  You’ll put unit tests in the src directory in each file with the code that they’re testing. The convention is to create a module named `tests` in each file to contain the test functions and to annotate the module with `cfg(test).`

### The Tests Module and #[cfg(test)]

The `#[cfg(test)]` annotation on the tests module tells Rust to Compile and run the test code only when you run `cargo test`, not when you run `cargo build`. This saves compiletime when you only want to build the library and save space in the resulting compiled artifact because the tests are not included

```rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

This code is the automatically generated test module. The attribute `cfg` stands for *configuration* and tells rust that the following item should only be included given a certain configuration option. In this case the configuration option is `test` which is provided by Rust for compiling and running tests. By using `cfg` attribute, Cargo compiles our test code only if we actively run the tests with `cargo test`. This includes help functions that might be within this module, in addition to the functions annotated with `#[test]`.


### Testing Private Functions

Rust privacy rule do allow you to test private functions.

Consider the code in Listing 11-12 with the private function internal_adder.

```rs
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

###### Listing 11-12: Testing a private function

Note that the `internal_adder` function is not marked as `pub`. Tests are just Rust code, and the `tests` module is just another module.

We bing all of the `test` module's parent's items into scope with `use super::*`, and then the test can call `internal_adder`.