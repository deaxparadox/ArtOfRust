# Updaing a Hash Map

Although the number of key and value pairs is growable, each unique key can only have one value associated with it at a time

When you want to change the data in a hash map, you have to decide how to handle the case when a key already has a value assigned. You could replace the old value with the new value, completely disregarding the old value. You could keep the old value and ignore the new value, only adding the new value if the key doesn’t already have a value. Or you could combine the old value and the new value.

### Overwriting a Value

If we insert a key and a value into a hash map and then insert the same key with a different value, the value associated with that key will be replaced. `insert` twice, the hash map will on contain one key/value pair because we're inserting the value for the Blue team's key both times.

```rs
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
```

This code will print `{"Blue": 25}`. The original of `10` has been overwritten.

----------

### Adding a Key and Value Only If a key isn't Present.

If's common to check whether a particular key already exists in the hash map with a value then take the following actions:

- if the key does exist insert in the hash map, the existing value should remain the way it is.
- If the key doesn't exist, insert it and a value for it.

Hash maps have a special API for this called `entry` that takes the key you want to check as a parameter. The return value of the `entry` methods is an enum called `Entry` that represents a value that might or might not exist.

Let’s say we want to check whether the key for the Yellow team has a value associated with it. If it doesn’t, we want to insert the value 50, and the same for the Blue team.

```rs
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

```

The `or_insert` method on `Entry` is defined to return a mutable reference to the vallue for the corresponding `Entry` key if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value.

Running the code will print `{"Yellow": 50, "Blue": 10}`. The first call to `entry` will insert the key for the Yellow team with the value 50 because the Yellow team doesn’t have a value already. The second call to `entry` will not change the hash map because the Blue team already has the value 10.

### Updating a Value Based on the Old Value

Another common use case for hash maps is to look up a key's value then update it based on the old value.

For instance, code that counts how many times each word appears in some text. We use a hash map with the words as keys and increment the value to keep track of how many times we’ve seen that word. If it’s the first time we’ve seen a word, we’ll first insert the value 0.

```rs
    let text = "hello world wonderfull world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
```

This code will print `{"world": 2, "hello": 1, "wonderful": 1}`.

The `split_whitespace` method returns an iterator over sub-slices, separated by whitespace, of the value in text. The `or_insert` method returns a mutable reference (`&mut V`) to the value for the specified key. Here we store that mutable reference in the `count` variable, so in order to assign to that value, we must first dereference `count` using the asterisk (`*`). The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.