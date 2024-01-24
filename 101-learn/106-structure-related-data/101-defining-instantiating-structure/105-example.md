# An Example Program Using Structs

Let's write a program that calculates the area of a rectangle.

Let’s make a new binary project with Cargo called *rectangles* that will take the width and height of a rectangle specified in pixels and calculate the area of the rectangle.


```rs
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

- On running this program using `cargo run`:

```bash
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/rectangles`
The area of the rectangle is 1500 square pixels.
```


The issue with this code is evident in the signature of area:

```rs
fn area(width: u32, height: u32) -> u32 {
```

- The `area` function is supposed to calculate the area of one rectangle, but the function we wrote has two parameters, and it’s not clear anywhere in our program that the parameters are related.
- It would be more readable and more manageable to group width and height together.

## Refactoring with Tuples

This shows another version of our program that uses tuples

```rs
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

-  Tuples let us add a bit of structure, and we’re now passing just one argument. 
- But in another way, this version is less clear: tuples don’t name their elements, so we have to index into the parts of the tuple, making our calculation less obvious.
- if we want to draw the rectangle on the screen, it would matter! We would have to keep in mind that `width` is the tuple index `0` and `height` is the tuple index `1`. 
- This would be even harder for someone else to figure out and keep in mind if they were to use our code

## Refactoring with Structs: Adding More Meaning

We use structs to add meaning by labeling the data.

- We can transform the tuple we’re using into a struct with a name for the whole as well as names for the parts.

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

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // this works because struct `rect1` is passed as reference
    // if we don't use reference we will get `move` error
    println!("Height: {}, Width: {}", rect1.height, rect1.width);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```