# Processing a Series of Items with Iterators


- [Home](../../README.md)
    - [Functional lanuage Features](../README.md)
        - [Closuers](../101-closures-anonymous-functions-that-capture-their-environment/README.md)
        - [Processing a Series of items](README.md)

----------

[<<< Functional Language features](../README.md) | [The Iterator Trait and the next Method](101-the-iterator-trait-and-the-next-method.md)


The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. 

In Rust, iterators are *lazy*, meaning they have no effect until you call methods that consume the iterator to use it up.

For example, the code in Listing 13-10 creates an iterator over the items in the vector `v1` by calling the `iter` method defined on `Vec<T>`. This code by itself doesn’t do anything useful.

```rs
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
```

###### Listing 13-10: Creating an iterator

The iterator is stored in the `v1_iter` variable.

In the example in Listing 13-11, we separate the creation of the iterator from the use of the iterator in the `for` loop. When the `for` loop is called using the iterator in `v1_iter`, each element in the iterator is used in one iteration of the loop, which prints out each value.

```rs
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

```

###### Listing 13-11: Using an iterator in a for loop

[<<< Functional Language features](../README.md) | [The Iterator Trait and the next Method](101-the-iterator-trait-and-the-next-method.md)