# References and Borrowing

A *reference* is like a pointer in that it's an address we can follow to access the data stored at that address; that data is owned by some other variable. 

- a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

```rs
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

- First, notice that all the tuple code in the variable declaration and the function return value is gone.
- Second, note that we pass `&s1` into `calculate_length` and, in its definition, we take `&String` rather than String.
- These ampersands represent *references*, and they allow you to refer to some value without taking ownership of it.

![Image 1](trpl04-05.svg)

### Note: The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *.

Let's take a closer look at the function call here:

```rs
let s1 = String::from("hello");

let len = calculate_length(&s1);
```

- The `&s1` syntax lets us create a reference that refers to the value of `s1` but does not own it.
- Because it does not own it, the value it points to will not be dropped when the reference stops being used.

- Likewise, the signature of the function uses `&` to indicate that the type of the parameter `s` is a reference. Let’s add some explanatory annotations:

```rs
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
```

The scope in which the variable `s` is valid is the same as any function parameter’s scope, but the value pointed to by the reference is not dropped when `s` stops being used, because `s` doesn’t have ownership. When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, because we never had ownership.

We call the action of creating a reference *borrowing*.


- [Mutable references](101-mutable-references.md)
- [Dangling references](102-dangling-reference.md)
- [Rules](103-rules.md)