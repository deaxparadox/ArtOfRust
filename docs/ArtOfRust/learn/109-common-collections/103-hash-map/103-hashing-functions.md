# Hashing Functions

By default, `HashMap` uses a hashing function called *SipHash* that can provide resistance to Denial of Service (DoS) attacks involving hash tables. 

- This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it. 
- If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher.
- A hasher is a type that implements the `BuildHasher` trait.