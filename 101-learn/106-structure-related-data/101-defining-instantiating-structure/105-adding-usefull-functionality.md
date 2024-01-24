# Adding Usefull Functionality with Derived Traits

It’d be useful to be able to print an instance of Rectangle while we’re debugging our program and see the values for all its fields.

- It we try using the `println!` macro as we have used in previous chapters. This won’t work, however.

```rs
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}
```

- When we compile this code, we get an error with this core message:

```rs
When we compile this code, we get an error with this core message:
```

Rust *does* include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. 

- To do that, we add the outer attribute `#[derive(Debug)] `just before the struct definition,

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

- Now when we run the program, we won’t get any errors, and we’ll see the following output:

```bash
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
rect1 is Rectangle { width: 30, height: 50 }

```

- When we have larger structs, it’s useful to have output that’s a bit easier to read; in those cases, we can use `{:#?}` instead of `{:?}` in the `println!` string. 
- In this example, using the `{:#?}` style will output the following:

```bash
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
rect1 is Rectangle {
    width: 30,
    height: 50,
}
```

Another way to print out a value using the `Debug` format is to use the `dbg!` macro, which takes ownership of an expression (as opposed to `println!`, which takes a reference), prints the file and line number of where that `dbg!` macro call occurs in your code along with the resultant value of that expression, and returns ownership of the value.

Note: Calling the `dbg!` macro prints to the standard error console stream (`stderr`), as opposed to `println!`, which prints to the standard output console stream (`stdout`)

Here’s an example where we’re interested in the value that gets assigned to the `width` field, as well as the value of the whole struct in `rect1`:

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```

- We can put `dbg!` around the expression `30 * scale` and, because `dbg!` returns ownership of the expression’s value, the `width` field will get the same value as if we didn’t have the `dbg!` call there. 
- We don’t want `dbg!` to take ownership of `rect1`, so we use a reference to `rect1` in the next call. 
- Here’s what the output of this example looks like:

```bash
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/rectangles`
[src/main.rs:10] 30 * scale = 60
[src/main.rs:14] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```