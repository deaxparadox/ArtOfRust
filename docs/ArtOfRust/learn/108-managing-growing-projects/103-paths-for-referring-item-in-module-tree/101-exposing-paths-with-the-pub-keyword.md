# Exposing Paths with the pub Keyword

Let’s return to the error that told us the hosting module is private. We want the `eat_at_restaurant` function in the parent module to have access to the `add_to_waitlist` function in the child module, so we mark the hosting module with the `pub` keyword.

Filename: src/lib.rs

```rs
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

###### Listing 7-5: Declaring the hosting module as pub to use it from eat_at_restaurant

Unfortunately, the code in Listing 7-5 still results in an error, as shown in Listing 7-6.

```bash
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: function `add_to_waitlist` is private
 --> src/lib.rs:9:37
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                                     ^^^^^^^^^^^^^^^ private function
  |
note: the function `add_to_waitlist` is defined here
 --> src/lib.rs:3:9
  |
3 |         fn add_to_waitlist() {}
  |         ^^^^^^^^^^^^^^^^^^^^

error[E0603]: function `add_to_waitlist` is private
  --> src/lib.rs:12:30
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^ private function
   |
note: the function `add_to_waitlist` is defined here
  --> src/lib.rs:3:9
   |
3  |         fn add_to_waitlist() {}
   |         ^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` due to 2 previous errors

```

###### Listing 7-6: Compiler errors from building the code in Listing 7-5

What happened? Adding the `pub` keyword in fron of `mod hosting` makes the module public. With this changed, if we can access `front_of_house`, we can access `hosting`. But the *contents* of `hosting` is still private; makiing the module public doesn't make its contents public. The `pub` keyword on a module only lets code in its ancestor modules refer to it, not access its inner code. Because modules are containers, there's not much we can do by only making the module public; we need to go further and choose to make one or more of the items within the module public as well.

The errors in Listing 7-6 say that the `add_to_wishlist` function is private. 

Let's also make the `add_to_waitlist` function public by adding the `pub` keword before the definition, as in Listing 7.7.

Filename: src/lib.rs

```rs
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

pub fn eat_at_restaurant() {
  // Absolute path
  create::front_of_house::hosting::add_to_waitlist();

  // Relative path
  front_of_house::hosting::add_to_waitlist();
}
```

###### Listing 7-7: Adding the `pub` keword to `mod hosting` and `fn add_to_waitlist` lets us call the function from `eat_at_restaurent`.
  
In the absolute path, we start with `crate`, the root of our crate’s module tree. The `front_of_house` module is defined in the crate root. While `front_of_house` isn’t public, because the `eat_at_restaurant` function is defined in the same module as `front_of_house` (that is, `eat_at_restaurant` and `front_of_house` are siblings), we can refer to `front_of_house` from `eat_at_restaurant`. Next is the hosting module marked with `pub`. We can access the parent module of hosting, so we can access hosting. Finally, the `add_to_waitlist` function is marked with `pub` and we can access its parent module, so this function call works!

In the relative path, the logic is the same as the absolute path except for the first step:rather than starting from the crate root, the path starts from `front_of_house`. The `front_of_house` module is defined within the same module as `eat_at_restaurent` is defined works. Then, because `hosting` and `add_to_waitlist` are marked with `pub`, the rest of the path works, and this function call is valid!

[<<< paths-for-referring-item-in-module-tree](README.md) ... [Starting Relative Paths with super](102-starting-relative-paths-with-super.md)