# The Option Enum and Its Advantages Over Null Values

The `Option` type encodes the very common scenario in which a value could be something or it could be nothing.

Rust doesn’t have the `null` feature that many other languages have. 

- `Null` is a value that means there is no value there. 
- In languages with `null`, variables can always be in one of two states: `null` or `not-null`.


As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is `Option<T>`


```rs
enum Option<T> {
    None,
    Some(T),
}
```

- The `Option<T>` enum is so useful that it’s even included in the `prelude`; you don’t need to bring it into scope explicitly. 
- Its variants are also included in the prelude: you can use `Some` and `None` directly without the `Option::` prefix. 
- The `Option<T>` enum is still just a regular enum, and `Some(T)` and `None` are still variants of type `Option<T>`.

The `<T>` syntax is a generic type parameter.

- For now, all you need to know is that `<T>` means that the `Some` variant of the `Option` enum can hold one piece of data of any type, and that each concrete type that gets used in place of `T` makes the overall `Option<T>` type a different type. 
- Here are some examples of using `Option` values to hold number types and string types:

```rs
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;  
```

- The type of `some_number` is `Option<i32>`. 
- The type of `some_char` is `Option<char>`, which is a different type. 
- Rust can infer these types because we’ve specified a value inside the `Some` variant. 
- For `absent_number`, Rust requires us to annotate the overall `Option` type: the compiler can’t infer the type that the corresponding `Some` variant will hold by looking only at `a` None value. 
- Here, we tell Rust that we mean for `absent_number` to be of type `Option<i32>`.