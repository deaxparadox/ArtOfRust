# Hash Maps and Ownership

For types that implement the `Copy` trait, like i32, the values are copied into the hashmap. For owned valeus like `String`, the value will be moved and hash map will be the owner of those values.

```rs
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
```


We aren't able to use the variables `field_name` and `field_value` afte they've been moved into the hashmap with the call to `insert`.

