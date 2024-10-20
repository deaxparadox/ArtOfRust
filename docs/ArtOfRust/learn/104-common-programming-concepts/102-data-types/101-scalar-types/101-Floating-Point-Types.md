# Floating-Point Types

- Rust also has two primitive types for floating-point numbers, which are numbers with decimal points.
- Rust’s floating-point types are `f32` and `f64`, which are 32 bits and 64 bits in size, respectively.
-  The default type is `f64` because on modern CPUs, it’s roughly the same speed as `f32` but is capable of more precision.
- All floating-poin types are signed.

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

[<<< Scalar Types](README.md) | [Numeric Operations >>>](102-Numeric-Operations.md)
