# What is a String?

Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`.

The `String` type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type. When Rustaceans refer to “strings” in Rust, they might be referring to either the `String` or the string slice `&str` types, not just one of those types


## Creating a new String

Creating a new, empty `String`:

```rs
    let mut s = String::new();
```

- This line creates a new empty string called `s`,  which we can then load data into. 

To create aa string with some initial data, we use the `to_string` method, which is available on any type that implements the `Display` trait, as string literals do.

```rs
    let data = "initial contents";

    let s = data.to_string();

    // the method also words to literal directly;
    let s = "initial content".to_string();
```

We can also use the funtion `String::from` to create a `String` from a string literal. The code is equivalent to the code from that use `to_string`.

```rs
    let s = String::from("initial contents");
```

Remember that strings are UTF-8 encoded, so we can include any properly encoded data in them,

```rs

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

```

- All of these are valid `String` values.

