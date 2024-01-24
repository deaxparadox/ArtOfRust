# Closure Type Inference and Annotation

There are more differences between functions and closures. Closures don’t usually require you to annotate the types of the parameters or the return value like `fn` functions do. Type annotations are required on functions because the types are part of an explicit interface exposed to your users.

Annotating the types for a closure would look like the definition shown in Listing 13-2. In this example, we’re defining a closure and storing it in a variable rather than defining the closure in the spot we pass it as an argument as we did in Listing 13-1.

Filename: src/main.rs

```rs
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
```

###### Listing 13-2: Adding optional type annotations of the parameter and return value types in the closure

Here we define a function that adds 1 to its parameter and a closure that has the same behavior, for comparison. We’ve added some spaces to line up the relevant parts. This illustrates how closure syntax is similar to function syntax except for the use of pipes and the amount of syntax that is optional:

```rs
// function with type annotations
fn  add_one_v1   (x: u32) -> u32 { x + 1 }

// closure with type annotations
let add_one_v2 = |x: u32| -> u32 { x + 1 };

// closure without type annotations
let add_one_v3 = |x|             { x + 1 };

// closure without type anntation and without curly brackets
let add_one_v4 = |x|               x + 1  ;
```

----------

For closure definitions, the compiler will infer one concrete type for each of their parameters and for their return value.

For instance, Listing 13-3 shows the definition of a short closure that just returns the value it receives as a parameter. This closure isn’t very useful except for the purposes of this example. Note that we haven’t added any type annotations to the definition. Because there are no type annotations, we can call the closure with any type, which we’ve done here with String the first time. If we then try to call example_closure with an integer, we’ll get an error.

Filename: src/main.rs

```rs
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);
```

###### Listing 13-3: Attempting to call a closure whose types are inferred with two different types

The compiler gives us this error:

```bash
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
error[E0308]: mismatched types
 --> src/main.rs:5:29
  |
5 |     let n = example_closure(5);
  |             --------------- ^- help: try using a conversion method: `.to_string()`
  |             |               |
  |             |               expected struct `String`, found integer
  |             arguments to this function are incorrect
  |
note: closure parameter defined here
 --> src/main.rs:2:28
  |
2 |     let example_closure = |x| x;
  |                            ^

For more information about this error, try `rustc --explain E0308`.
error: could not compile `closure-example` due to previous error
```

The first time we call `example_closure` with the `String` value, the compiler infers the type of `x` and the return type of the closure to be `String`. Those types are then locked into the closure in `example_closure`, and we get a type error when we next try to use a different type with the same closure

----------

[Capture The Environment with Closures](101-capturing-the-environment-with-closures.md) | [Capturing References or Moving Ownership](103-capturing-refrences-or-moving-ownership.md)