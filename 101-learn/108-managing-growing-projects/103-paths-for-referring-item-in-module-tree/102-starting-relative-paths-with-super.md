# Starting Relative Paths with super

We can construct relative paths that begin in the parent module, rather than the current module or the crate root, by using `super` at the start of the path. This is like starting a filesystem path with the `..` syntax. Using `super` allows us to reference an item that we known is in the parent module, which can make rearranging the module tree easier when the module closely related to the parent, but the parent might be moved elsewhere in the module tree someday.

Consider the code in Listing 7-8. The function `fix_incorrect_order` defined in the `back_of_house` modulle calls the function `delive_order` defined in the parent module by specifying the path to `deliver_order` starting with `super`:

Filename: src/lib.rs

```rs
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

The `fix_incorrect_order` function is in the `back_of_house` module, so we can use `super` to go to the parent module of `back_of_house`, which in this case is crate, the root. From there, we look for `deliver_order` and find it. Success! We think the `back_of_house` module and the deliver_order function are likely to stay in the same relationship to each other and get moved together should we decide to reorganize the crate’s module tree. Therefore, we used super so we’ll have fewer places to update code in the future if this code gets moved to a different module.


[<<< Exposing paths with the pub keyword](101-exposing-paths-with-the-pub-keyword.md) ... [Making Structs and Enums Public >>>](103-makeing-structs-and-enums-public.md)