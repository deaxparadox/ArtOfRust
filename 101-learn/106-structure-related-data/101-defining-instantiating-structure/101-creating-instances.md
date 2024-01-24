# Creating Instances from Other Instances with Struct Updates 

It's often usefull to create a new instance of a struct that includes most of the values from another instance, but changes some. You can do this using *struct updated syntax*.

```rs
fn main() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

we show how to create a new `User` instance in `user2` regularly, without the update syntax. 

- We set a new value for `email` but otherwise use the same values from `user1` that we created:

```rs
fn main() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

The syntax `..` specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.


```rs
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

The code creates an instance in `user2` that has a different value for `email` but has the same values for the `username`, `active`, and `sign_in_count` fields from `user1`. 

- The `..user1` must come last to specify that any remaining fields should get their values from the corresponding fields in `user1`, but we can choose to specify values for as many fields as we want in any order, regardless of the order of the fields in the struct’s definition.

Note that the struct update syntax uses `=` like an assignment; this is because it moves the data:

- In this example, we can no longer use `user1` as a whole after creating `user2` because the `String` in the `username` field of `user1` was moved into `user2`. 
- If we had given `user2` new String values for both `email` and `username`, and thus only used the `active` and `sign_in_count` values from `user1`, then `user1` would still be valid after creating `user2`