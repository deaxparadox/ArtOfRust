# Common Collections

Rust’s standard library includes a number of very useful data structures called *collections*. Most other data types represent one specific value, but collections can contain multiple values.

The data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.

Collections:

- A *vector* allows you to store a variable number of values next to each other.
- A *string* is a collection of characters.
- A *hash map* allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a *map*.