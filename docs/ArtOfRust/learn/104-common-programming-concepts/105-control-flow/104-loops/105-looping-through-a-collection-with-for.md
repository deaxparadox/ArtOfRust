# Looping Through a Collection with `for`

- You can choose to use the `while` construct to loop over the elements of collection, such as an array. For example, the loop in Listing 3-4 prints each element in the array `a`.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

###### Listing 3-4: Looping through each element of a collection using a while loop



- here, the code counts up through the elements in the array. It starts at index `0`, and them loops untils it reaches the final index in the array (that is, when `index < 5` is not longer `true`). Running this code will print every element in the array:


```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/loops`
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```

All five array values appear in the terminal, as expected. Even though `index` will reach a value of 5 at some point, the loop stops executing before trying to fetch a sixth value from the array.

----------

However, this approach is error prone; we could cause the program to panic if the index value or test condition is incorrect

As a more concise alternative, you can use a for loop and execute some code for each item in a collection. A `for` loop looks like the code

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

###### Listing 3-5: Looping through each element of a collection using a for loop

- When we run this code, we'll see the same output as in Listing 3-4.


----------

Here's what the countdown would loop like using a `for` loop and another method we've not yet taked about, `rev`, to reverse the range:

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```


###### Listing 3-5: Looping through each element of a collection using a for loop



[<<< Conditional loop with while loop](104-conitional-loops-with-while.md) | [Ownership >>>](../../../105-ownership/README.md)
