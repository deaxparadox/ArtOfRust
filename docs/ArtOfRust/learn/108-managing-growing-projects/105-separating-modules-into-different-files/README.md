# Separating Modules into Different Files

First, we’ll extract the `front_of_house` module to its own file. Remove the code inside the curly brackets for the `front_of_house` module, leaving only the `mod front_of_house`; declaration, so that *src/lib.rs* contains the code shown in Listing 7-21. Note that this won’t compile until we create the *src/front_of_house.rs* file in Listing 7-22.

Filename: stc/lib.rs


```rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

###### Listing 7-21 Declaring the `front_of_house` module whose body will be in *src/front_of_house.rs*

Next, place the code that was in the curly brackets into a new file name *src/front_of_house.rs*, as show in Listing 7-22. The compiller konws to look in this file because it came across the module declaration in the crate root with the name `front_of_house`.

Filename: src/front_of_house.rs

```rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

###### Listing 7-22: Definitions inside the `front_of_house` module in *src/front_of_house.rs*


----------

Note that you only need to load a file using a `mod` declaration once in your module tree. Once the compiler knows the file is part of the project (and knows where in the module tree the code resides because of where you’ve put the mod statement), other files in your project should refer to the loaded file’s code using a path to where it was declared.

----------

Next, we’ll extract the `hosting` module to its own file. The process is a bit different because hosting is a child module of `front_of_house`, not of the root module. We’ll place the file for `hosting` in a new directory that will be named for its ancestors in the module tree, in this case `src/front_of_house/`.

To start moving `hosting`, we change `src/front_of_house.rs` to contain only the declaration of the `hosting` module:

Filename: src/front_of_house.rs

```rs
pub mod hosting;
```


Then we create a `src/front_of_house` directory and a file `hosting.rs` to contain the definitions made in the hosting module:

Filename: src/front_of_house/hosting.rs

```rs
pub fn add_to_waitlist() {}
```

If we instead put `hosting.rs` in the src directory, the compiler would expect the `hosting.rs` code to be in a `hosting` module declared in the crate root, and not declared as a child of the `front_of_house` module. The compiler’s rules for which files to check for which modules’ code means the directories and files more closely match the module tree.



