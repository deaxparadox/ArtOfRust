# Hello world


### Create a Project Directory

Make a project directory to store you rust code.

Open a terminal and enter the following commands to make a projects directory and a directory for the “Hello, world!” project within the projects directory.

```bash
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

### Writing and Running a Rust Program

Make a new source file and call it main.rs. Rust files always end with the `.rs` extension.

If you're using more than one word in your filename, the convention is to use an underscore to separate them.

Open the *main.rs* file you just created and write the following code:

```rs
fn main() {
    println!("Hello, world!");
}
```

###### A program that prints `hello world!`

Save the file and go back to your terminal in the *~/projects/hello_world* directory. On linux, and engter the following commands to compile and run the file:

```bash
$ rustc main.rs
$./main
Hello, world!
```