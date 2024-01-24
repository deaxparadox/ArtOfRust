# Comments

To make code easy to understand, extra explanation is warranted. 

- It this case, programms leave *comments* in their source code that the compiler will ignore but people reading the source code may find useful.

Here's a simple comment:


```rust
// hello world
```

For comments that extend beyond a single line, you'll need to include `//` on each line, like this:

```rust
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.

```

Comments also be place at the end of lines containing code:

```rust
fn main() {
    let lucky_number = 7; // I’m feeling lucky today
}
```

But you'll more often use comment in this format, with the comment on a separate line above the code:


```rust
fn main() {
    // I’m feeling lucky today
    let lucky_number = 7;
}
```

[<<< Common Programming Concepts](README.md) | [Control Flow >>>](105-control-flow/README.md)