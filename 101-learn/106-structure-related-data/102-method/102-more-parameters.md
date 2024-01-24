# Methods with More Parameters

Let’s practice using methods by implementing a second method on the `Rectangle` struct. 

- This time we want an instance of `Rectangle` to take another instance of `Rectangle` and return `true` if the second Rectangle can fit completely within `self` (the first `Rectangle`); otherwise, it should return `false`.
- That is, once we’ve defined the can_hold method.

```rs
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

- The expected output would look like the following because both dimensions of `rect2` are smaller than the dimensions of `rect1`, but `rect3` is wider than `rect1`:

```
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

Let’s add the new can_hold method to the `impl` block:

```rs
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```