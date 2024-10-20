# Defining an Enum

Enumes gives you a way of saying a value is one of a possible set of values.

- defining an `IpAddrKind` enumeration and listing the possible kinds an IP address can be, `V4` and `V6`. These are the variants of the enum:

```rs
enum IpAddrKind {
    V4, 
    V6
}
```

- `IpAddrKind` is now a custom data type that we can use elsewhere in our code.

## Enum Values

We can create instances of each of the two variants of `IpAddrKind` like this:

```rs
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
```

```rs
enum IpAddrKind {
        V4, 
        V6
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    fn main() {
        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from(":;1"),
        };
    }
```

Representing the above concept using just an enum is more concis: rather than using an enum inside a struct, we can put data directly into each enum variant.

- This new definition of the `IpAddr` enum says that both `V4` and `V6` variants will have associated `String` values:

```rs
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

```

- Here, it’s also easier to see another detail of how enums work: the name of each enum variant that we define also becomes a function that constructs an instance of the enum. That is, `IpAddr::V4()` is a function call that takes a `String` argument and returns an instance of the IpAddr type.
- We automatically get this constructor function defined as a result of defining the enum.

There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data.

```rs
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

```

**You can put any kind of data inside an enum variant: strings, numeric types, or structs, for example.**

- You can even include another enum!
- Also, standard library types are often not much more complicated than what you might come up with.

Let’s look at another example of an enum in this one has a wide variety of types embedded in its variants.

```rs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

- This enum has four variants with different types:

1. `Quit` has no data associated with it at all.
2. `Move` has named fields, like a struct does.
3. `Write` includes a single `String`.
4. `ChangeColor` includes three `i32` values.