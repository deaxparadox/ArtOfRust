# Hello World

### Creating a Project Directory

Open a terminal and enter the following commands to make a *projects* directory and a directory for the "Hello World!" Project within the projects directory.

```bash
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

### Writing and Running a Rust Program

This is the Simple Rust program for for printing hello world.

- Create a file `main.rs`.
- Copy this code.

```rs
fn main() {
    println!("Hello, world!");
}
```

- Compile the code, using the following command.

```bash
$ rustc main.rs
```

- Run the executable.

```bash
$ ./main.rs
```

Congratulations! You've officially written a Rust program.


### Anatomy of a Rust Program

Let's review this "Hello World!" program in detail.

```rs
fn main() {

}
```

These lines define a function named `main`. The `main` function is special: it is always the first code that runs in every execute Rust program.

Here, the first lines declares a function named `main` that has no parameters and returns nothing. If there were parameters, they would go inside the parentheses `()`.


The function body is wrapped in `{}`. Rust requires curly brackets around all functions bodies. It's good style to place the curly bracket on the same line as the function declaration, adding one space in between.


The body of the `main` function holds the following code:

```rs
println!("Hello world!");
```

This line does all the work in this little program: it prints test to the screen. There are four important details to notice here.

**First**, Rust style is to indent with four space, not a tab.

**Second**, `println!` calls a Rust macro. It it had called a function instead, it would be entered as `println` (without the `!`). A `!` means that you're calling a macro instead of a normal function and that macros don't always follow the same rules as functions.

**Third**, "Hello world!" string. We passs this string as an argument to `println!`, and the string is printed on to the screen.

**Fourth**, we end the line with a semicolon (`;`), which indicates that this expression is over and the next one is ready to begin.