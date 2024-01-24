# Contants 

The `contants` are values that are bound to a name and are not allowed to change, but there are a few differences between contants and variables.

- Constants aren’t just immutable by default—they’re always immutable.
- You declare constants using the const keyword.
- Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.
- The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

- The compiler is able to evaluate a limited set of operations at compile time, which lets us choose to write out this value in a way that’s easier to understand and verify, rather than setting this constant to the value 10,800. 
- Constants are valid for the entire time a program runs, within the scope in which they were declared.

[<<< Variable and Mutability](101-Variables-and-Mutability.md) | [Shadowing >>>](103-Shadowing.md)