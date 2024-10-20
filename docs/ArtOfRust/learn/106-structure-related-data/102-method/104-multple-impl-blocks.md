# Multiple `impl` Blocks

Each struct is allowed to have multiple impl blocks.

```rs
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

- There’s no reason to separate these methods into multiple `impl` blocks here, but this is valid syntax.