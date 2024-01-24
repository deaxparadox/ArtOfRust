# Unit-Like Structs Without Any Fields

You can also define structs that don’t have any fields! These are called *unit-like structs* because they behave similarly to `()`.

- Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
- Here’s an example of declaring and instantiating a unit struct named `AlwaysEqual`:

```rs
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

- To define `AlwaysEqual`, we use the `struct` keyword, the name we want, and then a semicolon.

- No need for curly brackets or parentheses! Then we can get an instance of `AlwaysEqual` in the subject variable in a similar way: using the name we defined, without any curly brackets or parentheses.