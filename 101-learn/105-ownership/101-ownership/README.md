# What is Ownership ?

*Ownership* is a set of rules that govern how a Rust program manages memory.

Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. 

Rust manages memory through a system of ownership with a set of rules that the compiler checks. 
- If any of the rules are voilated, the programm won't compile.
- None of the features of ownership will slow down your program while it's running.

## Ownership Rules

Let's take a look at the ownership rules.

- Each value in Rust has an *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Variables Scope.

A scope is the range within a program for which an item is valid. 

Take the following variable:

```rust
let s = "hello";
```

- The variable `s` referes to string literal, where the value of the strnig is hardcoded into the text of our program.
- The variable is valid from the point at which it's declared until the end of the current *scope*.


```rust
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
```

###### Listing 4-1: A variable and the scope in which it is valid



Two important points in time here:

- When `s` comes into *scope*, it valid.
- It remains valid until it goes *out of* scope.

- [String type](101-string-type.md)
- [Memory Allocation](102-memory-allocation.md)
- [Ownership functions](103-ownership-functions.md)
- [Return values scope](104-return-values-scope.md)

[<<< Understanding Ownership](../README.md) | [String type >>>](101-string-type.md)