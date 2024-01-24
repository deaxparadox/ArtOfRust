# Defining and Instantiating Structs

- The pieces of a struct can be of different types.
- in struct you'll name each piece of data so it's clear what the values mean.


# Define a `struct`

To define a structure we use the `struct` keyword and name the entire struct.

- then inside curly brackets, we define the names and types of the pieces of data, which we call fields.

```rs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

```

- to use a struct after we've defined it, we created an *instance* of that struct by specifying concrete values for each of the fields.

- We create an instance by stating the name of the struct and then add curly brackets containing *key: value* pairs, where the keys are the names of the fields and the values are the data we want to store in those fields. 

- We don’t have to specify the fields in the same order in which we declared them in the struct. 

- **In other words, the struct definition is like a general template for the type, and instances fill in that template with particular data to create values of the type**

For example, we can declare a particular user:

```rs
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

- To get a specific value from a struct, we use dot notation.
- For example, to access this user’s email address, we use `user1.email`.

### Mutable instance 

- If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field. 
- Let change the value in the `email` field of a mutable `User` instance.


```rs
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

- we can construct a new instance of the struct s the last expression in the function body to implicitly return that new instance. 

we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.


```rs
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

```


## Using the Field Init Shorthand

Because the parameter names and the struct field names are exactly the same in Listing 

- we can use the field init shorthand syntax to rewrite build_user so it behaves exactly the same but doesn’t have the repetition of username and email

```rs
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

- [Creating instances](101-creating-instances.md)
- [Tuple struct](102-tuple-struct.md)
- [Unit like structure](103-unit-like-structure.md)
- [Ownership struct data](104-ownership-struct-data.md)
- [Adding functionality](105-adding-usefull-functionality.md)
- [Example](105-example.md)