# Vectors

The first collection type we’ll look at is `Vec<T>`, also known as a *vector*. Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. **Vectors can only store values of the same type**.

## Creating a New Vectors

To create a new empty vector, we call the `Vec::new` function:

```rs
let v: Vec<i32> = Vec::new();
```

Note that we added a type annotation here. Because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store. This is an important point. Vectors are implemented using generics.

For now, know that the `Vec<T>` type provided by the standard library can hold any type. When we create a vector to hold a specific type, we can specify the type within angle brackets.

The `Vec<T>` in `v` will hold elements of the `i32` type.

## Creating Vector with inital values

Create a `Vec<T>` with initial values and Rust will infer the type of value you want to store. Rust conveniently provides the `vec!` macro, which will create a new vector that holds the values you give it.

Let's create a new `Vec<i32>` that holds the value `1`, `2` and `3`. The integer tupe is `i32` because that's the default integer type:

```rs
    let v = vec![1, 2, 3];
```

Because we’ve given initial `i32` values, Rust can infer that the type of `v` is `Vec<i32>`, and the type annotation isn’t necessary.

## Updating a Vector

To create a vector and then add elements to it, we can use the `push` method:

```rs
let mut v = Vec::new();

v.push(1);
v.push(2);
v.push(3);
v.push(4);
```

As with any variable, if we want to be able to change its value, we need to make it mutable using the `mut` keyword. The numbers we place inside are all of type `i32`, and Rust infers this from the data, so we don’t need the `Vec<i32>` annotation.

## Reading Elements of Vectors

There are two ways to reference a value store in a vector:

- via indexing
- or using the get method.

#### Using indexing

```rs
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");
```

We use the inex vaue of `2` to get the third element because vectors are indexed by number, starting at zero. Using `&` and `[]` gives us a reference to the element at the index value.

Let's see another example, we will try to access non existing value:

```rs
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
```

When we try to access a non-exiting value. Rust panic, causes program to terminate.

```bash
Finished dev [unoptimized + debuginfo] target(s) in 0.57s
     Running `target/debug/Examples`
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100', src/collections/vector.rs:58:31
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

If you want your program to crash, when someone try to get value from non-existing range you can use this method.

#### Using get method

```rs
    let v = vec![1, 2, 3, 4, 5];

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
```

When we use the `get` method with the index passed as an argument, we get the `Option<T>` that we can use the `match`.

Let's try to access value from non-existing index range.

```rs
    let v = vec![1, 2, 3, 4, 5];

    let third = v.get(100);

    match third {
        Some(value) => println!("This third element using get method: {value}"),
        None => println!("Invalid index!"),
    }
```

It returns `None` without panicking.

```bash
Invalid index!
```

When you want to keep your program running or safe, then you can use this method.


## Iterating over the Values in a Vector

To access each element in a vector in turn, we would iterate through all of the elements rather than use indices to access one at a time.


#### Immutable reference

We will use `for` loop to get immutable references to each element in a vector of `i32` values and print them:

```rs
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
```


#### Mutable reference

We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements. The `for` loop will add 50 to each element:

```rs
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
```

To change the value that the mutable reference refers to, we have to use the `*` dereference operator to get to the value in `i` before we can use the `+=` operator.

Iterating over a vector, mutably and immutably, is safe because of the borrow checker's rules. If we attempted to insert or remove items in the `for` loop bodies, we would get a compiler error. 

The reference to the vector that the `for` loop holds prevents simultaneous modification of the whole vector.


## Dropping a Vector Drops Its Elements

Like any other `struct`, a vector is freed when it goes out of scope, 

```rs
{
    let v = vec![1, 2, 3, 4];
    // do stuff with v
} // v goes out of scope after ending curly bracket
```

When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up. The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.