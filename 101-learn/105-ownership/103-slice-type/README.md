# The Slice Type

*Slices* let you reference a contiguous sequence of elements in a collection rather than the whole collection.

- A slice is a kind of reference, so it does not have ownership.

Here’s a small programming problem: write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.

- Let’s work through how we’d write the signature of this function without using slices, to understand the problem that slices will solve:

```rs
fn first_word(s: &String) -> ?
```

- The `first_word` function has a `&String` as a parameter. 
- We don’t want ownership, so this is fine.
- But what should we return? We don’t really have a way to talk about `part` of a string. 
- However, we could return the index of the end of the word, indicated by a space. 

```rs
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

- Because we need to go through the `String` element by element and check whether a value is a space, we’ll convert our `String` to an array of `bytes` using the `as_bytes` method.

```rs
    let bytes = s.as_bytes();
```

- Next, we create an iterator over the array of bytes using the `iter` method:

```rs
    for (i, &item) in bytes.iter().enumerate() {
```

- For now, know that `iter` is a method that returns each element in a collection and that `enumerate` wraps the result of `iter` and returns each element as part of a tuple instead. The first element of the tuple returned from `enumerate` is the index, and the second element is a reference to the element. This is a bit more convenient than calculating the index ourselves.

- Because the enumerate method returns a tuple, we can use patterns to destructure that tuple.
- In the for loop, we specify a pattern that has `i` for the index in the tuple and `&item` for the single byte in the tuple. Because we get a reference to the element from `.iter().enumerate()`, we use & in the pattern.

- Inside the for loop, we search for the byte that represents the space by using the byte literal syntax.
- If we find space, we return the position.
- Otherwise, we return the length of the string by using s.len().

```rs
        if item == b' ' {
            return i;
        }
    }

    s.len()
```

- We now have a way to find out the index of the end of the first word in the string, but there’s a problem. We’re returning a `usize` on its own, but it’s only a meaningful number in the context of the `&String`. 
- In other words, because it’s a separate value from the `String`, there’s no guarantee that it will still be valid in the future.

```rs
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

- [String slices](101-string-slices.md)
- [Others](102-others.md)