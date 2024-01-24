# Providing New Names with the as Keyword

If we’re bringing two items with the same name into scope with use statements, because Rust doesn’t allow that. Listing 7-15 shows how to bring two `Result` types into scope that have the same name but different parent modules and how to refer to them.

```rs
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

###### Listing 7-15: Bringing two types with the same name into the same scope requires using their parent modules.

As you can see, using the parent modules distinguishes the two `Result` types. If instead we specified `use std::fmt::Result` and `use std::io::Result`, we’d have two` Result` types in the same scope and Rust wouldn’t know which one we meant when we used `Result`.


----------


There’s another solution to the problem of bringing two types of the same name into the same scope with `use:` after the path, we can specify `as` and a new local name, or alias, for the type. Listing 7-16 shows another way to write the code in Listing 7-15 by renaming one of the two `Result` types using `as`.

Filename: src/lib.rs

```rs
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}
```

###### Listing 7-16: Renaming a type when it’s brought into scope with the `as` keyword

In the second `use` statement, we chose the new name `IoResult` for the `std::io::Result` type, which won’t conflict with the `Result` from `std::fmt` that we’ve also brought into scope. Listing 7-15 and Listing 7-16 are considered idiomatic, so the choice is up to you!

[<<< Creating Idiomatic user path](101-creating-idiomatic-user-path.md) ... [Re-exporting Names with pub use >>>](103-re-exporting-names-with-pub-use.md)