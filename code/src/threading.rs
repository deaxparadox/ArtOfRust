use std::thread;
use std::time::Duration;
use std::os;

use crate::math::fibonacci;

pub fn run() {
    let builder = thread::Builder::new();
    
    let handler = builder.spawn(|| {
        fibonacci(40);
    }).unwrap();

    fibonacci(40);
    handler.join().unwrap();
}

pub fn run_multiple() {
    let mut join_handlers = Vec::new();

    for i in 0..16 {
        join_handlers.push(
            thread::Builder::new().spawn(move || {
                fibonacci(45);
            })
        )
    }

    while let Some(cur_thread) = join_handlers.pop() {
        cur_thread.unwrap().join().unwrap();
    }

}