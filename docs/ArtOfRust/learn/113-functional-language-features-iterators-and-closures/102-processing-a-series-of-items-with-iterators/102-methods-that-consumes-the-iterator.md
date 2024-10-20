# Methods that Consumes the Iterator

- [Home](../../README.md)
    - [Functional Language features](../README.md)
        - [Closures](../101-closures-anonymous-functions-that-capture-their-environment/README.md)
        - [Processing a series of Items](../102-processing-a-series-of-items-with-iterators/README.md)

----------

[<<< The Iterator trait and the next method](101-the-iterator-trait-and-the-next-method.md) | [Methods-that-produce-other-iterators >>>](103-methods-that-produce-other-iterators.md)

Methods that call `next` are called *consuming adaptors*, becuase calling them uses up the iterator.

One example is the `sum` method, which takes ownership of the iterator and iteratres through the items by repeatedly calling `next`, thus consuming the iterator. As it iterates through, it adds each item to a running total and returns the total when iteration is complete. Listing 13-13 has a test illustrating a use of the `sum` method:

Filename: src/lib.rs

```rs
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

```

###### Listing 13-13: Calling the sum method to get the total of all items in the iterator

We aren't allowed to use `v1_iter` after the call to `sum` because `sum` takes ownersip of the iterator we call it on.

[<<< The Iterator trait and the next method](101-the-iterator-trait-and-the-next-method.md) | [Methods-that-produce-other-iterators >>>](103-methods-that-produce-other-iterators.md)