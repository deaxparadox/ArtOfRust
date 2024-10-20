# Using Threads to Run Code Simultaneously

- An executed program’s code is run in a **process**, and the operating system will manage multiple processes at once.
- Within a program, you can also have independent parts that run simultaneously.
- The features that run these independent parts are called **threads**.

Because threads can run simultaneously, there’s no inherent guarantee about the order in which parts of your code on different threads will run.

- *Race conditions*, where threads are accessing data or resources in an inconsitent order.
- *Deadlocks*, where two threads are waiting for each other, preventing both threads from continuing
- Bugs that happend only in certian situations and are hard to reproduce and fix reliably.



[<<< README](README.md)... [New thread >>> ](101.1-creating-new-thread.md)