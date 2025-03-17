###### Extensible Concurrency with the sync and Send Traits

The following the two concurrency concepts that are embedded in the language: the `std::marker` traits `Sync` and `Send`.

### Allowing Tranference of Ownership Between Threads with Send

The `Send` marker trait indicates that ownership of values of the type implementing `Send` can be transffered between threads.

Rust's type system and traint bounds ensure that you can never accidentally send an `Rc<T>` value across threads unsafely.

Any type composed entirely of `Send` types is automaticallly marked as `Send` as well. Almost all primitive types are `Send`, aside from raw pointers.


### Allowing Access from Multiple Threads with Sync

The `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be referenced from mulitple threads.

In other words, any type `T` is Sync if `&T` (an immutable reference to `T`) is `Send`, meaning the reference can be sent safely to another thread. Similar to `Send`, primitive types are `Sync`, and types composed entirely of types that are `Sync` are also `Sync`.