# Defining Methods

Let’s change the `area` function that has a `Rectangle` instance as a parameter and instead make an `area` method defined on the `Rectangle` struct:

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

- The `&self` is actually short for `self: &Self`. Within an `impl` block, the type `Self` is an alias for the type that the `impl` block is for
- Note that we still need to use the `&` in front of the `self` shorthand to indicate that this method borrows the `Self` instance, just as we did in `rectangle: &Rectangle`.

- Methods can take ownership of `self`, borrow `self` immutably, as we’ve done here, or borrow `self` mutably, just as they can any other parameter.

-  If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use `&mut self` as the first parameter.

```rs
impl Rectangle {
        ...

        fn double_width(&mut self) {
            self.width *= 2;
        }
    }

    fn main() {
        let mut rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        
        rect1.double_width();

        println!("Width: {}", rect1.width);
        
        ...
    }
```

- the width is double

```bash
Width: 60
The area of the rectangle is 3000 square pixels.
```


- Having a method that takes ownership of the instance by using just `self` as the first parameter is rare; this technique is usually used when the method transforms `self` into something else and you want to prevent the caller from using the original instance after the transformation.

The main reason for using methods instead of functions, in addition to providing method syntax and not having to repeat the type of `self` in every method’s signature, is for organization.

- We’ve put all the things we can do with an instance of a type in one `impl` block rather than making future users of our code search for capabilities of `Rectangle` in various places in the library we provide.

Note that we can choose to give a method the same name as one of the struct’s fields.

- For example, we can define a method on `Rectangle` that is also named `width`:

```rs
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```

- Here, we’re choosing to make the `width` method return `true` if the value in the instance’s width field is greater than `0` and `false` if the value is `0`: we can use a field within a method of the same name for any purpose. 

- In main, when we follow `rect1.width` with `parentheses`, Rust knows we mean the `method width`. When we don’t use parentheses, Rust knows we mean the `field width`.

