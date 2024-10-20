# Methods that Produce Other Iterators

- [Home](../../README.md)
    - [Functional Language features](../README.md)
        - [Closures](../101-closures-anonymous-functions-that-capture-their-environment/README.md)
        - [Processing a series of Items](../102-processing-a-series-of-items-with-iterators/README.md)

----------

[<<< Method that consumes the iterators](102-methods-that-consumes-the-iterator.md) | [Using Closures that Capture Their Environment](104-using-closures-that-captures-their-environments.md)


*Iterator adaptors* are methods defined on the `Iterator` trait that don't consume the iterator. Instead they produce differents by changing some aspect of the original iterator.

Listing 13-14 shows an example of calling the iterator adaptor method `map`, which takes a closure to call on each item as the items are iterated through. The `map` method returns a new iterator that produces the modified items. The closure here creates a new iterator in which each item from the vector will be incremented by 1:

Filename: src/main.rs

```rs
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1);

```

###### Listing 13-14: Calling the iterator adaptor map to create a new iterator

However, this code produces a warning:

```bash
$ cargo run
   Compiling iterators v0.1.0 (file:///projects/iterators)
warning: unused `Map` that must be used
 --> src/main.rs:4:5
  |
4 |     v1.iter().map(|x| x + 1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: iterators are lazy and do nothing unless consumed
  = note: `#[warn(unused_must_use)]` on by default

warning: `iterators` (bin "iterators") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/iterators`

```

To fix this warning and consumethe iterator, we'll use the `collect` method. This method consumes the iterator and collects the resulting values into a collection data type.

In Listing 13-15, we collect the results of iterating over the iterator that’s returned from the call to `map` into a vector. This vector will end up containing each item from the original vector incremented by 1.

Filename: src/main.rs

```rs
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

```

###### Listing 13-15: Calling the `map` method to create a new iterator and then calling the `collect` method to consume the new iterator and create a vector

Because `map` takes a closure, we can specify any operation we want to perform on each item.

[<<< Method that consumes the iterators](102-methods-that-consumes-the-iterator.md) | [Using Closures that Capture Their Environment](104-using-closures-that-captures-their-environments.md)