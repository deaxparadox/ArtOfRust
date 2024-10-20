# The Iterator Trait and the next Method



- [Home](../../README.md)
    - [Functional Language features](../README.md)
        - [Closures](../101-closures-anonymous-functions-that-capture-their-environment/README.md)
        - [Processing a series of Items](../102-processing-a-series-of-items-with-iterators/README.md)

----------


[<<< Processing a series of items with iterators](README.md) | [Methods that Consumes the iterator >>>](102-methods-that-consumes-the-iterator.md)


All iterators implements a trait named `Iterator` that is defined in the standard library. The definition of the trait looks like thid:

```rs
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

```

Notice this definition uses some new syntax: `type Item` and `Self::Item`, which are defining an *associated type* with this trait.

This code says implementing the `Iterator` trait requires that you also define an `Item` type, and this `Item` type is used in the return type of the `next` method. In other words, the `Item` type will be the type returned from the iterator.

The `Iterator` trait only requres implementors to define one method: the `next` method, which returns one item of the iteratora at a time wrapped in `Some` and, when iteration is over, returns `None`.

We can call the `next` method on iterators directly; demontrating what values are returned from repeated calls to `next` on the iterator created from the vector.

Filename: src/lib.rs

```rs
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

```

Listing 13-12: Calling the `next` method on an iterator

Note that we need to make `v1_iter` mutable: calling the `next` method no an iterator chagnes internal state of the iterator uses to keep track of where it is in sequence. In other words, this code *consumes*, or uses up, the iterator. Each call to `next` eats up an item from the iterator. 

We didn't need to make `v1_iter` mutable when we used a `for` loop because the loop took ownership of `v1_iter` and made it mutable behind the scenes.

The values we get from the calls to `next` are immutable references to the values in the vector. 

The `iter` method produces an iterator over immutable references.


If we want to create an iterator that takes ownership of `v1` and returns owned values, we call `into_iter` instead of `iter`.

Similarly, it we want to iteratr over mutable references, we cal call `iter_mut` instead of `iter`.

[<<< Processing a series of items with iterators](README.md) | [Methods that Consumes the iterator >>>](102-methods-that-consumes-the-iterator.md)