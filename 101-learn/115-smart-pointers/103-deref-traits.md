# The deref traits


- [Smart Pointers](README.md)
    - [<<< Enabling recursive types with boxes](102-enabling-recursive-types-with-boxes.md)
        - [Deref Trait >>>](103-deref-traits.md)

----------

```rust
pub trait Deref {
    type Target: ?Sized;

    // Required method
    fn deref(&self) -> &Self::Target;
}
```

Used for immutable dereferencing operations, like `*v`.

In addition to being used for explicit derefrencing operations with the (unary) `*` operator in immutable contexts, `Deref` is also used implicitly by the compiler in many circumtances. The mechanism is called `"Deref coercion"`. 

In mutable contexts `DerefMut` is used and mutable deref coercion similarly occurs.


----------

**Warning**: Deref coercion is a powerful language feature which has far-reaching implications for each type that implements `Deref`. The compiler will silently insert calls to `Deref::deref`. For this reason, one should be carefull about implementing `Deref` and only do so when deref coercion is desirable.


----------


Type that implement `Deref` or `DerefMut` are often called "smart pointers" and the mechanism of deref coercion has been specifically designed to facilitate the pointer-like behaviour that name suggests.

Often, the purple of a "smart pointer" type is to change the ownership sementics of a contained value (for example, `Rc` or `Cow`) or the storage sementics of a contained value (for example, `Box`).