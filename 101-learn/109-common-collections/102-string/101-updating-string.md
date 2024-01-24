<!-- .push_str() -->
<!-- .push() -->
<!-- + -->
<!-- format! -->

# Updating a String

A `String` can grow in size and its content can change, just like the contents of a `Vec<T>`, if you push more data into it. In addition, you can conveniently use the `+` operator or the `format!` macro to concatenate `String` values.

## Appending to a String with push_str and push

#### `push_str` method
We can grow a `String` by using the `push_str` method to append a string slice:

```rs

    let mut s = String::from("foo");
    s.push_str("bar");

```

- `s` will contain `foobar`. The `push_str` method takes a string slice because we don't necessarily want to take ownership of the parameter. For example, we want to be able to use `s2` after appending its contents to `s1`.

```rs

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

```

- If the `push_str` method took ownership of `s2`, we wouldn’t be able to print its value on the last line. However, this code works as we’d expect!

#### `push` method

The `push` method takes a single character as a parameter and adds it to the `String`, add the letter "l" to the `String` using the `push` method:

```rs

let mut s = String::from("lo");
    s.push('l');

```

- As a result, `s` will contain `lol`.