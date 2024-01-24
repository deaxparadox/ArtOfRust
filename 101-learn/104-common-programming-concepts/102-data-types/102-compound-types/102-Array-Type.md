# The Array Type

Another way to have collection of multiple values is with an *array*. 

- Unlike a tuple, every element of an array must have the same type.
- arrays in Rust have a fixed length.

We write the values in an array as a comma-separated list inside square brackets:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

- Arrays are useful when you want your data allocated on the stack rather than the heap or when you want to ensure you always have a fixed number of elements.

- arrays are more useful when you know the number of elements will not need to change. 

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

- `i32` is the type of each element.
- After the semicolon, the number `5` indicates the array contains five elements.

You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:

```rust
lst a = [3; 5];
```

- The array named a will contain 5 elements that will all be set to the value 3 initially. 
- This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.


[<<< Tuple Types](101-Tuple-Type.md) | [Accessing Array Elements >>>](103-Accessing-Array-Elements.md)