# The **String** type

String type are immutable

- This type manages data allocated on the heap and as such is able to store an amount of text that is unkonwn to us at compile time.
- You can create a `String` from a string literal using the `from` function.

```rust
let s = String::from("hello");
```

- The double colon `::` operator allows us to namespace this particular `from` function under the `String` type rather than using some sort of name like `string_from`

- This kind of string can be mutated:

```ts
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`

```

[<<< Ownership](README.md) | [Memory Allocation >>>](102-memory-allocation.md)