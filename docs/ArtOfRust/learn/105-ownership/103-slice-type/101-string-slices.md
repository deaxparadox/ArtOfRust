# String Slices

A *string slice* is a refrence to part of a `String`, and it looks like this:

```rs
pub fn main() {
    let s = String::from("Hello world!");

    let hello = &s[0..5];
    // let world = &s[6..11];          // does not include space
    let world = &s[5..11];          // include space

    println!("{}{}", hello, world);
}
```

- hello is a reference to a portion of the `String`, specified in the extra `[0..5]` bit. 
- We create slices using a range within brackets by specifying `[starting_index..ending_index]`, where `starting_index` is the first position in the slice and `ending_index` is one more than the last position in the slice. 
- Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to `ending_index` minus `starting_index`.
- So, in the case of `let world = &s[6..11];`, world would be a slice that contains a pointer to the byte at index 6 of `s` with a length value of `5`.

![Image 6](trpl04-06.svg)

Rust's `..` range synxtax, if you want to start at index 0, you can drop the value before the two periods, 

```rs
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

If your slice includes the last bytes of the `String`, you can drop the trailing number.

```rs
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

you can also drop both values to take a slice of the entire string.

```rs
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

Let's write `first_word` to return a slice:

```rs
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

```

- The type that signifies "string slice" is written as `&str`.


## String Literals as Slices

```rs
let s = "Hello, world!";
```

- The type of `s` here is `&str`: it's a slice pointing to that specific point of the binary.
- This is also why string literals are immutable; `&str` is an immutable reference.


## String Slices as Parameters

You can take slices of literals and `String` values leads us to one more important on `first_word`, and that's its signature

```rs
fn first_word(s: &String) -> &str {
```

You can also write the the signature as:

```rs
fn first_word(s: &str) -> &str {
```

- it allow us to use the same function on both `&String` and `&str` values.


Defining a function to take a string slice instead of a reference to a `String`:

```rs
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```