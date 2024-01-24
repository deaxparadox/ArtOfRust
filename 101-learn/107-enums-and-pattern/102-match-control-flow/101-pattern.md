Patterns That Bind to Values

Another usefull feature of match arms is that they can bind to the parts of the values that match the pattern. This is how we can extract values out of enum variants.

- We can add this information to our enum by changing the `Quarter` variant to include a `UsState` value stored inside it,

```rs
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {}
```

- In the match expression for this code, we add a variable called `state` to the pattern that matches values of the variant `Coin::Quarter`. 
- When a `Coin::Quarter` matches, the state variable will bind to the value of that quarter’s `state`. 
- Then we can use `state` in the code for that arm,