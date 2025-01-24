# Using Threads to Run Code Simultaneously

- An executed program’s code is run in a **process**, and the operating system will manage multiple processes at once.
- Within a program, you can also have independent parts that run simultaneously.
- The features that run these independent parts are called **threads**.

Splitting the computation in your program to multiple threads to run mulitple tasks at the same time can improve performance, but it also adds complexity. Because threads can run simultaneously, there’s no inherent guarantee about the order in which parts of your code on different threads will run. This can lead to problems such as:

- *Race conditions*, where threads are accessing data or resources in an inconsitent order.
- *Deadlocks*, where two threads are waiting for each other, preventing both threads from continuing
- Bugs that happend only in certian situations and are hard to reproduce and fix reliably.


Programming languages implement threads in a few different ways, and many operating systems provide an API the language can call for creating new threads. The Rust standard library uses a *1:1* model of thread implementation, whereby a program uses one operating system thread per one language thread.

[<<< README](README.md)... [New thread >>> ](102-creating-new-thread.md)