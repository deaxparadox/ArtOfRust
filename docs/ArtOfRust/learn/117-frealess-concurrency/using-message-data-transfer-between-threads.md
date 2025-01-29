# Using Message Passing to Transfer Data between threads

The accomplish message-sending concurrency, where threads communicate by sending each other messages containing data. Rust standard library provides an implementation of *channels*. A channel is a general programming concept by which data is sent from one thread to another.

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