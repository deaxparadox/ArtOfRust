# Traits as Parameters

We known how to define an implement traits, we can now use traits to define functions that accept many different types. We'll use the `Summary` trait we implemented on the `NewsArticle` and `Tweet` types. To define a `notify` function that calls the `summarize` method on its item parameter, which is of some type that implemenets the `Summary` trait. To do this, we use `impl Trait` syntax like below:

```rs
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

Instead of concrete type for the `item` parameter, we specify the `impl` keyword and the trait name. This parameter accepts any type that implements that specified trait. In the body of `notify`, we can call any methods on `item` that come from the `Summary` trait, such as `summarize`. We can call `notify` and pass in any instance of `NewsArticle` or `Tweeet`. Code that calls the function with any other type, such as a `String` or an `i32`, won't compile because those type don't implement `Summary`.

### Trait Bound Syntax

The `impl Trait` syntax works for straightforward cases but it actually syntax sugar for a longer form known as a *trait bound*; it looks like this:

```rs
pub fn notify<T: Summary>(item &T) {
    println!("Breaking news! {}", item.summarize());
}
```

This longer form is equivalent to the example in the previous section but is more verbose. We place trait bounds with the declaration of the generictype parameter after a colon and inside angle brackets.

The `impl Trait` syntax is convenient and makes for more consice code in simple cases, while the fuller trait bound syntax can express more complexity in other cases. For example, we can have two parameters that implement `Summary`. Doing so with the `impl Trait` syntax looks like this:

```rs
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
```

Using `impl Trait` is appropriate if we want this function to allow `item1` and `item2` to have different types (as long as types implement `Summary`). It we want to force both parameters to have the same type, however, we must use a trait bound, like this:

```rs
pub fn notify<T: Summary>(item: &T, item: &T) {}
```

The generic type `T` specified as the type of the `item1` and `item2` parameters constrains the function such that the concrete type of the value passed as an argument for `item1` and `item2` must be the same.


### Specifying Multiple Trait Bounds with the + Syntax

We can also specify more than one trait bound. Say we wanted `notify` to use display formatting as well as `summarize` on `item`: we specify in the `notify` definition that `item` must implement both `Display` and `Summary`. We can do so using the `+` syntax:


```rs
pub fn notify(item: &(impl Summary + Display)) {}
```

The `+` syntax is also valid with trait bounds on generics types:

```rs
pub fn notify<T: Summary + Display>(item: &T) {}
```

With the two trait bounds specified, the body of `notify` can call `summarize` and use `{}` to format `item`.

### Clearer Trait Bounds with where Clauses.

Using to many trait bound has it own downsides. Each generic has its own trait bounds, so function with multiple generic type parameters can contain lots of trait bound information between the function's name and it's paramter list, making the function signature hard to read. For this reason, Rust has alternate syntax for specifying trait bounds inside a **`where`** clause after the function signature. So instead of writing this:

```rs
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

we can use a where clause, like this:

```rs
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```

This function’s signature is less cluttered: the function name, parameter list, and return type are close together, similar to a function without lots of trait bounds.

[<<< default-implemetations](101-default-implemetations.md) ... [returning-types-that-implement-traits >>>](103-returning-types-that-implement-traits.md)