# Ownership of Struct Data

In the User struct definition,we used the owned `String` type rather than the `&str` string slice type. 

- This is a deliberate choice because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.

It’s also possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes, 

- Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.