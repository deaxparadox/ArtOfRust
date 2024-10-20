# Managing Growing Projects with Packages, Creates and Modules

Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:

- **Packages**: A Cargo feature that lets you build, test, and share crates
- **Crates**: A tree of modules that produces a library or executable
- **Modules** and use: Let you control the organization, scope, and privacy of paths
- **Paths**: A way of naming an item, such as a struct, function, or module

Topics:

- [Packages and Crates](101-packages-and-crates/README.md)
- [Defining Modules to Control Scope and Privacy](102-defining-modules-to-control-scopes-and-privacy/README.md)
- [Paths for Referring to an Item in the Module Tree](103-paths-for-referring-item-in-module-tree/README.md)
- [Bringing Paths into Scope with the user Keyword](104-bringing-paths-into-scope-with-the-user-keyword/README.md)
- [Separating Modules into Different Files](105-separating-modules-into-different-files/README.md)