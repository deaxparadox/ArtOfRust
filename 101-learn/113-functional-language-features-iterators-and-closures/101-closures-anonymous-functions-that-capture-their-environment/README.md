# Closures: Anonymous Functions that Capture their Environment

- [Home](../../README.md)
    - [Functional lanuage Features](../README.md)
        - [Closuers](README.md)
        - [Processing a Series of items](../102-processing-a-series-of-items-with-iterators/README.md)

Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.

syntax for create closures 

```rs
    // defining a closure
    let fn_name = |p1, p2, ...| {
        // block of code
    }

    // calling a closure
    fn_name();
```


- [Capture The Environment with Closures](101-capturing-the-environment-with-closures.md)
- [Closure Type Inference and Annotation](102-closure-type-inference-and-annotation.md)
- [Capturing References or Moving Ownership](103-capturing-refrences-or-moving-ownership.md)
- [Moving captured values out of closures and the fn traits](104-moving-captured-values-out-of-closures-and-the-fn-traits.md)

[<<< Functional language features](../README.md) | [Capture The Environment with Closures](101-capturing-the-environment-with-closures.md)