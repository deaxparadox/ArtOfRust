# Using Message Passing to Transfer Data between threads

The accomplish message-sending concurrency. Rust standard library provides an implementation of *channels*. A channel is a general programming concept by which data is sent from one thread to another.

A channel has two halves: a transmitter and a receiver. One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages. A channel is said to be *closed* if either the transmitter or receiver half is dropped.

Here, we'll work up to a program that has one thread to generate values and send them down a channel, and another thread  that will receive the values and print them out. We'll be sending simple values between threads using a channel to illustrate the feature. Once you're familiar with the technique, you could use channels for any threads that need to communicate between each other such as a chat system of a system where many threads perform parts of a calculation and send the parts to one threads that aggregates the results.

We'll create a channel but not do anything with it. Note that this won't compile yet because Rust can't tell what type of values we want to send over the channel.

```rs
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
```

###### Listing 16-6: Creating a channel and assigning the two halves to `tx` and `rx`

- Created a new channel using the `mpsc::channel` function, `mpsc` stands for *multiple producer, single consumer*. 
- This function returns a tuple, the first element of which is the sending end--the transmitter--and the second element is the receiving end--the receiver.

Let's move the transmitting end into a spawned thread and have it send one string so the spawned thread is communicating with the main thread.

```rs
use std::sync::mpsc;
use std::thread;


fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}
```

###### 16-7: Moving `tx` to a spawned thread and sending "hi".

To receive the value form the spawned thread, we have to setup a receiver in main thread. Let's setup a receiver.

```rs
use std::sync::mpsc;
use std::thread;

pub fn first() {
    let (tx, rx) = mpsc::channel();


    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
```
###### Listing 16-8: Receiving the value “hi” in the main thread and printing it

The receiver has two useful methods: `recv` and `try_recv`.

- `recv`: short for *receive*, which will block the main thread's execution and wait until value is sent down the channel. Once a value is sent, `recv` will return it in a `Resullt<T, E>`. When the transmitter closes, `recv` will return an error to signal that no more values will be coming.

- `try_recv`: This method doesn't block, but instead return a `Result<T, E>` immediately: an `Ok` value holding a message if one is available, and and `Error` value if there aren't any messages this time.

### Channels and Ownership Transference

Let’s do an experiment to show how channels and ownership work together to prevent problems: we’ll try to use a val value in the spawned thread after we’ve sent it down the channel.

```rs
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {val}");
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
```

###### Listing 16-9: Attempting to use `val` after we've sent it down the channel

Here, we try to print `val` after we've sent it down the channel via `tx.send`. Allowing this would be a bad idea: once the value has been sent to another thread, that thread could modify or drop it before we try to use the value again. Potentially, the other thread's modifications could cause errors or unexpected results due to inconsistent or nonexistent data. However, Rust gives us an error if we try to compile the code:

```bash
$ cargo run
   Compiling message-passing v0.1.0 (file:///projects/message-passing)
error[E0382]: borrow of moved value: `val`
  --> src/main.rs:10:26
   |
8  |         let val = String::from("hi");
   |             --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
9  |         tx.send(val).unwrap();
   |                 --- value moved here
10 |         println!("val is {val}");
   |                          ^^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
error: could not compile `message-passing` (bin "message-passing") due to 1 previous error

```

The `send` function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it. This stops us from accidently using the value again after sending it; the ownership system checks that everything is okay.


### Sending Multiple Values and Seeing the Receiver Waiting

Let's send mulitple messages from the spawned thread:


```rs
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
```

###### Listing 16-10: Sending multiple messages and pausing between each


### Creating Mulitple Producers by Cloning the Transmitter

Let' create multiple threads that all send values to the same values. We can do so by cloning the transmitter:

```rs
use std::sync::mpsc;
use std::thread;


let (tx, rx) = mpsc::channel();


let tx1 = tx.clone();
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(time::Duration::from_millis(500));
    }
});

// let tx2 = tx.close();
thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(time::Duration::from_secs(1));
    }
});

for mes in rx {
    println!("Recevied: {mes}");
}


```

###### Listing 16-11: Sending multiple messages from multiple producers

The output might differ:

```bash
Recevied: hi
Recevied: more
Recevied: from
Recevied: messages
Recevied: the
Recevied: thread
Recevied: for
Recevied: you
```